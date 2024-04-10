use std::sync::{Arc};
use tokio::sync::Mutex;

pub struct MyThing {
    shared: Arc<Mutex<String>>,
}

pub trait Thing {
    fn new(shared: Arc<Mutex<String>>) -> Self;
}

impl Thing for MyThing {
    fn new(shared: Arc<Mutex<String>>) -> Self {
        MyThing {
            shared,
        }
    }
}
