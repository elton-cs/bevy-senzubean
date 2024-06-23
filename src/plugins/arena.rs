use bevy::{ecs::query, prelude::*};

use super::torii::{self, ToriiResource};
use torii_grpc::types::schema::Model;

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_arena);
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct Arena {
    x: u32,
    y: u32,
}

fn spawn_arena(torii: Res<ToriiResource>, query: Query<&Arena>, mut commands: Commands) {
    if query.iter().count() == 0 {
        let model = torii.data.models.iter().find_map(|model| {
            if model.name == "Arena" {
                Some(model)
            } else {
                None
            }
        });

        let (x, y) = get_entity_data(model);
        match (x, y) {
            (0, 0) => {}
            (x, y) => {
                info!("Spawning arena of size {}x{}", x, y);
                commands.spawn(Arena { x, y });
            }
        }
    }
}

fn get_entity_data(model: Option<&Model>) -> (u32, u32) {
    if let Some(model) = model {
        let data = model.members[1].ty.as_struct().unwrap();

        let x = data.children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        let y = data.children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        (x, y)
    } else {
        (0, 0)
    }
}
