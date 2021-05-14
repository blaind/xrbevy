use bevy::prelude::*;
use xrbevy::{is_already_running, ExampleScenePlugin};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    if is_already_running() {
        //return;
    }

    App::build()
        .add_plugin(ExampleScenePlugin)
        .add_plugin(bevy::openxr::OpenXRHandTrackingPlugin)
        .run();
}
