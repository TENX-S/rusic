use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Debug)]
pub struct Timeline {
    info: Arc<Mutex<Info>>
}

#[derive(Debug)]
struct Info<S=String, F=f64> {
    title: S,
    label: S,
    ratio: F,
    elapsed: F,
    duration: F,
}

