use std::sync::atomic::AtomicUsize;

pub struct Timeout {
    pub current: AtomicUsize,
}
