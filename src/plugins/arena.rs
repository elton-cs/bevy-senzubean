use bevy::prelude::*;

use super::torii::ToriiResource;
use torii_grpc::types::schema::Model;

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, render_setup)
            .add_systems(Update, spawn_arena);
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

fn render_setup(
    mut commands: Commands,
    query: Query<&Arena>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let arena = query.single();

    let texture: Handle<Image> = asset_server.load("stone_ground.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 27,
        },
    ));
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
