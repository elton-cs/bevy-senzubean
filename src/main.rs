use bevy::prelude::*;
use bevy_senzubean::plugins::{arena::ArenaPlugin, torii::ToriiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ToriiPlugin)
        .add_plugins(ArenaPlugin)
        .add_systems(Startup, spawn_camera_setup)
        .run();
}

fn spawn_camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
