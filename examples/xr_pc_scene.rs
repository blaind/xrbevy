use bevy::prelude::*;

use xrbevy::ExampleScenePlugin;

fn main() {
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();

    App::new().add_plugin(ExampleScenePlugin).run();
}
