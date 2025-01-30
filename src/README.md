# My first FruitMountain-like in Bevy

## 0. Before the course

[Install Rust](https://www.rust-lang.org/tools/install)

Clone this repo and run this command (the first compile is can take a while)
```
cargo run
```

Optional: Install the rust-analyzer extension for VSCode

You are good to go !

## 1. Basics

I will assume you already know the basics of working with entities, components and systems in Bevy.
If that's not the case, feel free to ask for help.

## 2. Setting up the game

If you don't know what Fruit Mountain is, it's a simple physics game where you throw fruits into a bowl, where similar fruits combine into larger fruit.

The goal is to get the biggest fruit without any fruit falling out of the bowl

![Fruit](../images/fruit.png "Fruit")

We will start by setting up the bowl, the camera and some lighting.

### 2.1 The Camera

For now we just want a camera looking at the center.
We can do that by spawning an entity with the `Camera3d` component and a `Transform`
```rust
    Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y), // Position the camera at (1, 1, 1) looking at (0, 0, 0)
```

### 2.2 The Bowl

Now that we can see the (empty) world, we can load the bowl 3d model I generously provide by using the `SceneRoot` component like so:
```rust
    SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("bowl/bowl.glb"))),
```

By default meshes do not interact with the physics engine, so we need to add collider and RigidBody components

```rust
    ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh), // Create a collider that follows the triangle of the mesh
    RigidBody::Static, // Set this entity as static, aka immovable
```

Now if we run our game, we should see our bowl in the middle.

### 2.3 The lights

Right now our bowl is in the shadows, because no light exists in our world.

To remedy this we can add an entity with one of the many light components that are available in Bevy.

Personally I will use a `DirectionalLight` to emulate the Sun, but feel free to experiment [with more lights](https://bevyengine.org/examples/3d-rendering/lighting/):

```rust
    DirectionalLight {
        illuminance: light_consts::lux::OVERCAST_DAY, // Set the intensity
        shadows_enabled: true, // We want cool shadows
        ..default() // We want the rest as default
    },
    Transform::from_rotation(Quat::from_rotation_x(-1.)), // Point the sun down
```

## 3. The fruits

Now that we can see our well lit bowl, we need our fruit.

We will start with simply spawning fruit, and then introduce more gameplay mechanics later.

### 3.1. Spawning fruit

If we want to spawn a fruit everytime we click with our mouse, we will need to detect input in a `Update` system.

Once you added that system to our `App`, you can detect inputs like so:

```rust
fn throw_fruit(
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        println!("It works !");
        // Spawn our fruit here
    }
}
```

Once you confirmed the system works as expected, we can add the system parameters `mut meshes: ResMut<Assets<Mesh>>`, `mut materials: ResMut<Assets<StandardMaterial>>` that are necessary to spawn meshes from scratch.

Do so by spawning an entity with these entities

```rust
    Mesh3d(meshes.add(Sphere::new(0.1))), // Create a sphere mesh
    MeshMaterial3d(materials.add(StandardMaterial { // Create a new material to color the sphere
        base_color: Color::WHITE,
        ..default()
    })),
    Collider::sphere(0.1), // Set the collider shape
    RigidBody::Dynamic, // Set this entity as dynamic, meaning it will be able to move around on it's own
```

## 4. What now ?

From there there is a lot we can do.

- Collision Detection to merge the fruit
- Throwing the fruits from the camera
- Rotating the camera
- Score system
- Fail conditions
- And much much more !

Feel free to do what you want and to ask for help if you need it.

