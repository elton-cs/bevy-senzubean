use bevy::prelude::*;
use bevy_senzubean::plugins::{arena::ArenaPlugin, torii::ToriiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ToriiPlugin)
        .add_plugins(ArenaPlugin)
        .run();
}
