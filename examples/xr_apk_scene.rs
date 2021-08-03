use bevy::prelude::*;
use xrbevy::{is_already_running, ExampleScenePlugin};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    if is_already_running() {
        // in android, main() can be called on resume
        // will create a new thread, but the process and existing App will be present
        println!("Already running - quit call to main()");
    } else {
        App::new()
            .add_plugin(ExampleScenePlugin)
            .add_plugin(bevy_openxr::OpenXRHandTrackingPlugin)
            .run();
    }
}
