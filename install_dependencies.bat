mkdir repos

for %%s in ("android-ndk-rs", "bevy", "bevy_rapier", "gfx", "openxrs", "wgpu", "wgpu-rs") do (
    git -C repos clone https://github.com/blaind/%%s.git
    git -C repos/%%s fetch origin
    git -C repos/%%s checkout bevy_openxr
)

git -C repos clone https://github.com/blaind/bevy_openxr
