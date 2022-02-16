mod helper;
use bevy::prelude::*;
use bevy_physics_take2::{Body, ColliderSphere, Mass, PhysicsPlugin};
use helper::HelperPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Physics Sandbox".to_string(),
            vsync: false, // just for testing
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HelperPlugin)
        // our plugin
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
        .add_system(setup_level)
        .run();
}

fn setup_level(
    mut ev_reset: EventReader<helper::ResetEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for _ in ev_reset.iter() {
        info!("Reset");

        let ball_material = materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("checker_red.png")),
            ..Default::default()
        });
        let ball_mesh = meshes.add(Mesh::from(bevy::render::mesh::shape::UVSphere {
            radius: 0.5,
            ..Default::default()
        }));

        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                mesh: ball_mesh.clone(),
                material: ball_material.clone(),
                ..Default::default()
            })
            .insert(Body {
                linear_velocity: Vec3::new(10.0, 0.0, 0.0),
                elasticity: 0.0,
                friction: 0.01,
                mass: Mass::Value(0.1),
                ..Default::default()
            })
            .insert(ColliderSphere::new(0.5))
            .insert(helper::Reset)
            .insert(Name::new("Sphere"));

        //Ground
        let ground_size = 1000.0;
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: ground_size,
                    sectors: 32,
                    stacks: 32,
                })),
                transform: Transform::from_xyz(0.0, -ground_size, 0.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(Body {
                elasticity: 1.0,
                friction: 0.5,
                mass: Mass::Static,
                ..Default::default()
            })
            .insert(ColliderSphere::new(ground_size))
            .insert(helper::Reset)
            .insert(Name::new("Ground"));
    }
}

fn setup(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(helper::CameraController::default())
        .insert(Name::new("Camera"));

    // light
    helper::spawn_light(&mut commands);
}
