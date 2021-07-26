# Building for Oculus Quest 2

## Prequisites

Prequisite installation:

    sudo apt-get install make openjdk-11-jre g++ libudev-dev libasound2-dev gcc
    cargo install cargo-apk

If you have not setup udev yet, copy udev rules for Oculus Quest 2

    # run both as root
    echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="2833", ATTR{idProduct}=="0186", MODE="0660", GROUP="plugdev"' > /etc/udev/rules.d/51-android.rules
    udevadm control --reload-rules && udevadm trigger


## Oculus OpenXR plugin

You must download `libopenxr_loader.so` from [Oculus OpenXR Mobile SDK](https://developer.oculus.com/downloads/package/oculus-openxr-mobile-sdk/) file. It should be stored to `libs/arm64-v8a` folder. It is at path `OpenXR/Libs/Android/arm64-v8a/Release` in the zip file.

    $ ls -al libs/arm64-v8a/libopenxr_loader.so
    -rwxrwxr-x 1 user user 10639048 feb  13 14:46 libs/arm64-v8a/libopenxr_loader.so

    $ file libs/arm64-v8a/libopenxr_loader.so
    ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked, BuildID[sha1]=..., with debug_info, not stripped

## Android SDK

You need android SDK & NDK to build .apk.

Append to `.bashrc` (and change paths if versions have changed...)

    export ANDROID_SDK_ROOT="$HOME/Android/Sdk"
    export ANDROID_NDK_ROOT="$HOME/Android/Sdk/ndk/22.0.7026061"

Unzip https://developer.android.com/studio/command-line/sdkmanager to `$ANDROID_SDK_ROOT/cmdline-tools`

After that, install required sdk's:

    ./cmdline-tools/bin/sdkmanager --sdk_root=$ANDROID_SDK_ROOT "ndk;22.0.7026061"
    ./cmdline-tools/bin/sdkmanager --sdk_root=$ANDROID_SDK_ROOT "build-tools;30.0.3"
    ./cmdline-tools/bin/sdkmanager --sdk_root=$ANDROID_SDK_ROOT "platforms;android-30"
    ./cmdline-tools/bin/sdkmanager --sdk_root=$ANDROID_SDK_ROOT "platforms;android-29"

## Run the example

    cargo apk run --example xr_apk_scene --release

or

    make run_xr_apk

If successful, it should output something like this:

```
    Finished release [optimized] target(s) in 32.78s
 'lib/arm64-v8a/libxr_apk_scene.so'...
 'lib/arm64-v8a/libc++_shared.so'...
 'lib/arm64-v8a/libopenxr_loader.so'...
Verifying alignment of /.../Bevy OpenXR wgpu.apk (4)...
      49 AndroidManifest.xml (OK - compressed)
    1152 assets/textures/uv-small.png (OK)
  319920 lib/arm64-v8a/libxr_apk_scene.so (OK - compressed)
15689936 lib/arm64-v8a/libc++_shared.so (OK - compressed)
17515002 lib/arm64-v8a/libopenxr_loader.so (OK - compressed)
Verification succesful
Performing Incremental Install
Serving...
Unknown command: install-incremental
Performing Streamed Install
Success
Starting: Intent { act=android.intent.action.MAIN cmp=rust.example.xr_apk_scene/android.app.NativeActivity }
```
