use mockall::mock;
use tokio::sync::Mutex;
use std::sync::Arc;
use test_mockall_tokio::Thing;

mock! {
    pub MyThing {}

    impl Thing for MyThing {
        fn new(shared: Arc<Mutex<String>>) -> Self;
    }
}