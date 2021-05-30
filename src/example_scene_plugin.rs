use bevy_openxr::prelude::*;
use bevy::prelude::*;

pub struct ExampleScenePlugin;

impl Plugin for ExampleScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Msaa { samples: 2 }) // FIXME openxr crashes if samples > 1
            .add_plugin(bevy_openxr::OpenXRPlugin)
            .add_plugins(DefaultPlugins)
            .add_plugin(bevy_openxr_core::OpenXRCorePlugin)
            .add_plugin(bevy_openxr::OpenXRWgpuPlugin)
            .add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let small_uv = asset_server.load("textures/uv-small.png");
    let small_uv_material = materials.add(StandardMaterial {
        base_color_texture: Some(small_uv.clone()),
        unlit: false,
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1., 1., 1.),
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    commands.spawn_bundle(XRCameraBundle::default());

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
        material: small_uv_material,
        transform: Transform::from_xyz(0., 0.8, -0.7),
        ..Default::default()
    });

    let color_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    // for now, do not draw cubes as they slow down
    return;

    for y in -3..3 {
        for x in -3..3 {
            for z in -3..3 {
                if x == 0 && z == 0 {
                    continue;
                }

                let spacing = 0.5;

                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: color_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * spacing,
                        y as f32 * spacing,
                        z as f32 * spacing,
                    ),
                    ..Default::default()
                });
            }
        }
    }
}
