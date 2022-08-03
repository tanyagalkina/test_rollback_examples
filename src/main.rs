#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_workaround {
    use std::panic::AssertUnwindSafe;

    use futures::FutureExt;

    #[tokio::test]
    async fn just_a_test_in_the_module() {
        assert_eq!("haha", "haha");
    }

    struct TestStruct {}

    impl Default for TestStruct {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TestStruct {
        pub fn new() -> Self {
            Self {}
        }
        // in case of using this cleanup as drop() cleanup function must NOT be able to panic!!!
        async fn cleanup(&mut self) {
            println!("I am cleaning created resources...");
            // if cleanup panics then the program aborts with "thread panicked while panicking. aborting.""
            println!("I am not allowed to panic cause I am in the panic thread!!!");
            //panic!("I am not allowed to panic here cause I am called while unwinding");
        }
        pub async fn test_function(&mut self) {
            println!("I am async test function");
            panic!("I am allowed to panic because I am a test ...")
        }
    }

    //learn about what each thing does
    impl Drop for TestStruct {
        fn drop(&mut self) {
            futures::executor::block_on(self.cleanup());
            //tokio::runtime::Handle::current().block_on(self.cleanup());
        }
    }

    //trying to use async Drop for cleaning the resources
    #[tokio::test]
    async fn try_to_use_async_drop_test() {
        let mut test_struct = TestStruct {};
        test_struct.test_function().await;
    }

    async fn reproduce_panic_with_return_value() -> Result<(), Box<dyn std::error::Error>> {
        //here test function body
        //panic!("I am panicking cause I am a test");
        assert!("haha" == "hoho");
        Ok(())
    }

    async fn reproduce_panic_while_panicking() {
        //here test function body
        panic!("I am panicking and not returning aynthing cause I am a test")
    }

    //standard template with catch_unwind
    #[tokio::test]
    async fn reproduce_panic_while_panicking_test() {
        let test_result = AssertUnwindSafe(reproduce_panic_while_panicking())
            .catch_unwind()
            .await;
        if test_result.is_err() {
            println!("I am rollback function of the failed test");
            println!("I am cleaning the resources ..");
            panic!("Can I panic here? or you want to abort then?");
            // assert!(deleting_existing_db("example", &my_client).await.unwrap());
            // assert!(std::fs::rename("./test/example-import.json.done", "./test/example-import.json").is_ok());
        }
        assert!(test_result.is_ok());
    }

    //Using return value Result instead of catch_unwind
    #[tokio::test]
    async fn panic_with_return_value_result() {
        let test_result = reproduce_panic_with_return_value().await;
        println!("I am here");
        if test_result.is_err() {
            println!(
                "Return value test failed, but I will never go here, cause the test body panicked"
            );
            panic!("I cannot clean the resources, cause I will never be called");
        }
        assert!(test_result.is_ok());
    }

    #[tokio::test]
    async fn just_a_test() {
        assert_ne!(42, 21);
    }

    #[tokio::test]
    async fn just_a_test_with_a_message() {
        assert_ne!(42, 42, "The values are equal");
    }

    #[tokio::test]
    async fn just_one_more_test() {
        assert_ne!(42, 84);
    }
}
