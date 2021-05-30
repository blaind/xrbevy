# Introduction

Proof-of-concept of getting [OpenXR](https://www.khronos.org/openxr/) rendering support for [Bevy](https://github.com/bevyengine/bevy) game engine using [gfx-rs](https://github.com/gfx-rs/gfx/) abstractions.

![Example](docs/screenshot.webp) 

Please note that the code with this PoC has not been merged upstream yet. Consider this code as a PoC where you can get things running, but there will be unstability.

## Install

### Windows

Tested with Oculus Quest 2 using Oculus Link in Virtual Desktop mode (must be enabled before starting app).

Clone this repository and run [install_dependencies.bat](./scripts/install_dependencies.bat) to download required dependencies (patched bevy, wgpu, openxrs crates, etc.):

    git clone https://github.com/blaind/xrbevy.git
    cd xrbevy
    .\scripts\install_dependencies.bat

Run the example scene using [Rust](https://www.rust-lang.org/).

    cargo run --example xr_pc_scene --release

If you don't already have an openxr loader, the process will complain about missing `openxr_loader.dll` file. One way to have a loader is to use a Khronos loader:

1. Navigate to https://github.com/KhronosGroup/OpenXR-SDK-Source/releases/
1. Look for the latest release and a file called `openxr_loader_windows-[version].zip`
1. Copy `openxr_loader_windows\x64\bin\openxr_loader.dll` to `xrbevy` folder (from zip)

The loader must also be configured to point into correct runtime. For Oculus, first install [Oculus app](https://www.oculus.com/setup/) and change the active runtime through a registry editor. Open a terminal as administrator, and run:

    # Print current values
    reg query HKEY_LOCAL_MACHINE\SOFTWARE\Khronos\OpenXR\1

    # Modify ActiveRuntime value to Oculus:
    reg add HKEY_LOCAL_MACHINE\SOFTWARE\Khronos\OpenXR\1 /v ActiveRuntime /d "C:\Program Files\oculus\Support\oculus-runtime\oculus_openxr_64.json"

If errors, see [troubleshooting](#troubleshooting)

In future, to get the latest changes:

    .\scripts\update_dependencies.bat

### Ubuntu

**Note: tested only with Monado in virtual desktop mode, not with real hardware**

Install needed dependencies:

    sudo apt-get install make libudev-dev libasound2-dev

Clone this repository and run a [Makefile](./Makefile) command to download required dependencies (patched bevy, wgpu, openxrs crates, etc.):

    git clone https://github.com/blaind/xrbevy.git
    cd xrbevy
    make download_dependencies

Run the example scene using [Rust](https://www.rust-lang.org/):

    cargo run --example xr_pc_scene --release

If you don't already have an openxr loader, the process will complain about missing `openxr_loader.dll` file. Please try first One way to install a loader is to use [Monado](https://monado.dev/) loader. Ubuntu installation:

    sudo add-apt-repository ppa:monado-xr/monado
    sudo apt-get update
    sudo apt-get install libopenxr-loader1
    
If you still get the error after retrying cargo run, try:

    sudo ln -s /usr/lib/x86_64-linux-gnu/libopenxr_loader.so.1 /usr/local/lib/libopenxr_loader.so && sudo ldconfig

In addition to a loader, you'll need an OpenXR runtime: either virtualized like Monado, or real like **maybe** (untested) SteamVR. For Monado, see https://monado.freedesktop.org/getting-started.html#monado. If you find a way to run with real hardware, please contact author or make a pull request to document the steps here.

If errors, see [troubleshooting](#troubleshooting)

In future, to get the latest changes:

    make update_dependencies

### Mac OS

Currently untested. Please make a pull request if you get it working.

### Oculus Quest 2

See [Oculus installation instructions](./docs/install_oculus.md)

# Background

Targets:
1. Demonstrate that OpenXR rendering is possible on Rust/GFX-RS/Bevy -ecosystem
1. Start the discussion with ecosystem participants about proper way to implement OpenXR support for Rust
1. Eventually have the basic building blocks in an easy-to-use state for building XR apps on top of bevy, or implementing XR support for other rendering engines


For technical details, see [docs/architecture.md](docs/architecture.md):

<a href="docs/architecture.md"><img src="docs/screenshot0.png" height="200"></a>
<a href="docs/architecture.md"><img src="docs/screenshot3.png" height="200"></a><br />

Bevy OpenXR plugin:

* bevy_openxr: https://github.com/blaind/bevy_openxr

Tracked core pull requests at upstream. Note that while these are the current changes to make this PoC work, the code in these might not be eventual solution to land into the repositories.

* bevy: https://github.com/bevyengine/bevy/pull/2166
* wgpu-rs: https://github.com/gfx-rs/wgpu-rs/pull/910
* wgpu: https://github.com/gfx-rs/wgpu/pull/1387
* gfx-rs: https://github.com/gfx-rs/gfx/pull/3761

A few miscellanceous / support crates:
* bevy_rapier: https://github.com/dimforge/bevy_rapier/pull/69 (only required for kinematics in the example)
* android-ndk-rs: https://github.com/rust-windowing/android-ndk-rs/pull/138 (needed for bundling .so for Oculus Quest 2)

# Related material

Further reading - some links that have helped in getting this PoC working:

* https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html
* https://github.com/Ralith/openxrs/blob/master/openxr/examples/vulkan.rs
* https://github.com/GodotVR/godot_openxr

# Community

Check out [Bevy Discord](https://discord.gg/gMUk5Ph) channel `#xr`

# Troubleshooting

## ERROR_FORM_FACTOR_UNAVAILABLE

    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ERROR_FORM_FACTOR_UNAVAILABLE', repos\gfx\src\backend\vulkan\src\xr.rs:63:14

Solution: Make sure that your device is connected

## OpenXR loader not found

    Could not load openxr loader. Make sure that you have openxr_loader.dll (Windows), libopenxr_loader.dylib (MacOS) or libopenxr_loader.so (Linux) in the library load path

Solution: follow the platform-specific installation instructions, and make sure that one of the above files is present in library load path (or current path).

## Unknown wgpu/OpenXR extension

```
ADD INSTANCE EXTENSION! "VK_KHR_external_memory_capabilities"
ADD INSTANCE EXTENSION! "VK_KHR_external_fence_capabilities"
ADD INSTANCE EXTENSION! "VK_KHR_external_semaphore_capabilities"
ADD DEVICE EXTENSION! "VK_KHR_external_memory"
ADD DEVICE EXTENSION! "VK_KHR_external_memory_win32"
thread 'main' panicked at 'Unknown wgpu/OpenXR extension, add to openxr_extensions! macro: Unknown', repos\gfx\src\backend\vulkan\src\xr.rs:233:26
```

Solution: some of the extensions is missing from xr code. Please open a new issue to this repository, or make a pull request directly into: https://github.com/blaind/gfx/blob/bevy_openxr/src/backend/vulkan/src/xr.rs#L326 (last part with `openx_extensions!` macro). The format is as:

    (VK_KHR_EXTERNAL_SEMAPHORE_WIN32, "VK_KHR_external_semaphore_win32", 1048576),
     ^^^ all uppercase                 ^^ format as specific by OpenXR   ^^ pow2

For the pow2 number, take the max/latest number and multiply it by two to get a next value.

## No OpenXR runtime found

    Error [GENERAL |  | OpenXR-Loader] : RuntimeManifestFile::FindManifestFiles - failed to find active runtime file in registry
    Error [GENERAL | xrEnumerateInstanceExtensionProperties | OpenXR-Loader] : RuntimeInterface::LoadRuntimes - unknown error
    Error [GENERAL | xrEnumerateInstanceExtensionProperties | OpenXR-Loader] : RuntimeInterface::LoadRuntimes - failed to load a runtime
    Error [GENERAL | xrEnumerateInstanceExtensionProperties | OpenXR-Loader] : Failed to find default runtime with RuntimeInterface::LoadRuntime()
    Error [GENERAL | xrEnumerateInstanceExtensionProperties | OpenXR-Loader] : Failed querying extension properties

Solution/Linux: Modify `/etc/xdg/openxr/1/` files based on your runtime (see Google)

Solution/Windows: See the `ActiveRuntime` part of the Windows setup before

