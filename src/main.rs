//! A shader that uses dynamic data like the time since startup.
//! The time data is in the globals binding which is part of the `mesh_view_bindings` shader import.

use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(camera_orbit_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // cube
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     material: materials.add(CustomMaterial {}),
    //     ..default()
    // });

    // sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 4,
            })
            .unwrap(),
        ),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(CustomMaterial {}),
        ..default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraOrbit);
}

fn camera_orbit_system(time: Res<Time>, mut query: Query<(&CameraOrbit, &mut Transform)>) {
    let delta_time = time.delta_seconds();
    let rotation_speed: f32 = 45.0; // degrees / second

    for (_, mut transform) in query.iter_mut() {
        let current_rotation = Quat::from_rotation_y(delta_time * rotation_speed.to_radians());
        transform.rotation = current_rotation * transform.rotation;
        transform.translation = current_rotation.mul_vec3(transform.translation);
    }
}

#[derive(Component)]
struct CameraOrbit;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
