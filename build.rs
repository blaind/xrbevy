use std::path::Path;

fn main() {
    //#[cfg(target_os = "android")] <-- TODO: make this work
    if !Path::new("libs/aarch64-linux-android/libopenxr_loader.so").exists() {
        panic!("libs/aarch64-linux-android/libopenxr_loader.so is missing - see readme for instructions. This applies to Quest 2, if building for PC you can edit build.rs and commen out this line for now");
    }
}
