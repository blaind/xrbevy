mod example_scene_plugin;
pub use example_scene_plugin::*;

use std::sync::atomic::{AtomicBool, Ordering};
static IS_STARTED: AtomicBool = AtomicBool::new(false);

pub fn is_already_running() -> bool {
    if IS_STARTED.load(Ordering::SeqCst) {
        return true;
    }
    IS_STARTED.store(true, Ordering::SeqCst);
    false
}
