mod mocks;

use std::sync::Arc;
use tokio::sync::Mutex;
use test_mockall_tokio::Thing;
use crate::mocks::MockMyThing;


#[tokio::test]
async fn main() {
    let thing = MockMyThing::new(Arc::new(Mutex::new("".to_string())));
}