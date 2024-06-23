use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_senzubean::{plugins::torii::ToriiPlugin, torii::run_torii_client};
use starknet_ff::FieldElement;
use tokio::runtime::Builder;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ToriiPlugin)
        .run();
}
