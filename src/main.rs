use avian3d::PhysicsPlugins;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup() {} //Will be run at the start

fn update() {} //Will be run every frame
