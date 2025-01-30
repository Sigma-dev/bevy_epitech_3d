use avian3d::{
    prelude::{
        Collider, ColliderConstructor, ColliderConstructorHierarchy, CollisionStarted, Gravity,
        Mass, PhysicsDebugPlugin, RigidBody,
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
        .add_systems(Update, (throw_fruit, rotate, collisions))
        .insert_resource(Gravity::default())
        .run();
}

#[derive(Component)]
struct Rotate {
    rate: f32,
}

#[derive(Component, PartialEq, Eq)]
struct Fruit(FruitType);

#[derive(Clone, Copy, PartialEq, Eq)]
enum FruitType {
    Grape,
    Orange,
    Watermelon,
}

const FRUIT_ORDER: [FruitType; 3] = [FruitType::Grape, FruitType::Orange, FruitType::Watermelon];

fn collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    fruit_q: Query<(&Transform, &Fruit)>,
) {
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        println!("Z");
        let Ok((t1, f1)) = fruit_q.get(*e1) else {
            continue;
        };
        println!("U");
        let Ok((t2, f2)) = fruit_q.get(*e2) else {
            continue;
        };
        println!("a");
        if f1 != f2 {
            continue;
        }
        let fruit_index = FRUIT_ORDER.iter().position(|f| *f == f1.0).unwrap();
        if fruit_index + 1 == FRUIT_ORDER.len() {
            continue;
        }
        let next_fruit = &FRUIT_ORDER[fruit_index + 1];
        spawn_fruit(
            next_fruit,
            (t1.translation + t2.translation) / 2.,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        commands.entity(*e1).despawn();
        commands.entity(*e2).despawn();
    }
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
            Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            rotation: Quat::from_rotation_x(-1.),
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
    fruit_type: &FruitType,
    position: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let radius;
    let color;
    match fruit_type {
        FruitType::Grape => {
            radius = 0.05;
            color = Color::srgb(1., 0., 0.5)
        }
        FruitType::Orange => {
            radius = 0.1;
            color = Color::srgb(0.7, 0.2, 0.1)
        }
        FruitType::Watermelon => {
            radius = 0.2;
            color = Color::srgb(0., 1., 0.)
        }
    }
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(radius))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })),
        Collider::sphere(radius),
        RigidBody::Dynamic,
        Mass(1.),
        Transform::from_translation(position),
        Fruit(fruit_type.clone()),
    ));
}

fn throw_fruit(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        /* let radius = 0.05;
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
        )); */
        spawn_fruit(
            &FruitType::Grape,
            Vec3::new(0., 0.5, 0.),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}
