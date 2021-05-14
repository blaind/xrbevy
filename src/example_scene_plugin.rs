use bevy::openxr::XRCameraBundle;
use bevy::prelude::*;

pub struct ExampleScenePlugin;

impl Plugin for ExampleScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Msaa { samples: 2 }) // FIXME openxr crashes if samples > 1
            .add_plugins_with(DefaultPlugins, bevy::openxr::add_plugins_fn)
            .add_plugin(bevy::openxr_core::OpenXRCorePlugin)
            .add_startup_system(setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let small_uv = asset_server.load("textures/uv-small.png");
    let small_uv_material = materials.add(StandardMaterial {
        albedo_texture: Some(small_uv.clone()),
        unlit: false,
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn(LightBundle {
            light: Light {
                color: Color::rgb(1., 1., 1.),
                ..Default::default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .spawn(XRCameraBundle::default());

    for y in -3..3 {
        for x in -3..3 {
            for z in -3..3 {
                if x == 0 && z == 0 {
                    continue;
                }

                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(x as f32 * 0.5, y as f32 * 0.5, z as f32 * 0.5),
                    ..Default::default()
                });
            }
        }
    }
}
