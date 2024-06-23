use bevy::{ecs::query, prelude::*};
use bevy_senzubean::plugins::{arena::ArenaPlugin, senzubean::SenzubeanPlugin, torii::ToriiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ToriiPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(SenzubeanPlugin)
        .add_systems(Startup, spawn_camera_setup)
        // .add_systems(Update, list_entities)
        .run();
}

fn spawn_camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn list_entities(query: Query<Entity>) {
    let num = query.iter().count();
    println!("Number of entities: {}", num);
}
