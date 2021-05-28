mkdir repos

for %%s in ("android-ndk-rs", "bevy", "bevy_rapier", "gfx", "openxrs", "wgpu", "wgpu-rs") do (
    git -C repos/%%s pull
)

git -C repos/bevy_openxr pull
git pull