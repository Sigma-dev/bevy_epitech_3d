use avian3d::{
    prelude::{
        Collider, ColliderConstructor, ColliderConstructorHierarchy, Gravity, Mass,
        PhysicsDebugPlugin, RigidBody,
    },
    PhysicsPlugins,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_fruit, rotate))
        .insert_resource(Gravity::default())
        .run();
}

#[derive(Component)]
struct Rotate {
    rate: f32,
}

fn rotate(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut rotate_q: Query<(&mut Transform, &Rotate)>,
) {
    for (mut transform, rotate) in rotate_q.iter_mut() {
        let rotation = time.delta_secs() * rotate.rate;
        if keys.pressed(KeyCode::KeyD) {
            transform.rotate_local_y(rotation);
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.rotate_local_y(-rotation);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Transform::default(), Rotate { rate: 10. }))
        .with_child((
            Camera3d::default(),
            Transform::from_translation(Vec3::splat(1.)).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            rotation: Quat::from_rotation_x(-3.14 / 4.),
            ..default()
        },
    ));
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("bowl/bowl.glb"))),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
    ));
}

fn spawn_fruit(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let radius = 0.05;
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(radius))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1., 0., 1.),
                ..default()
            })),
            Collider::sphere(radius),
            RigidBody::Dynamic,
            Mass(1.),
            Transform::from_xyz(0., 0.5, 0.),
        ));
    }
}
