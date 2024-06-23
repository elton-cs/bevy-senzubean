use bevy::prelude::*;
use bevy_senzubean::plugins::{
    arena::ArenaPlugin, player::PlayerPlugin, senzubean::SenzubeanPlugin, torii::ToriiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ToriiPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(SenzubeanPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_camera_setup)
        // .add_systems(Update, list_entities)
        .run();
}

fn spawn_camera_setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(100.0, 50.0, 1.0);
    camera.projection.scale = 0.25;
    commands.spawn(camera);
}

fn list_entities(query: Query<Entity>) {
    let num = query.iter().count();
    println!("Number of entities: {}", num);
}
