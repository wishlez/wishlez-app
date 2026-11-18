use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct ReopenLauncher(AtomicBool);

impl ReopenLauncher {
    pub fn request(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    pub fn take_request(&self) -> bool {
        self.0.swap(false, Ordering::SeqCst)
    }
}
