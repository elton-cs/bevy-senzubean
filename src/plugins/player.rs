use super::torii::ToriiResource;
use bevy::{prelude::*, transform};
use torii_grpc::types::schema::Model;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_player, render_player_spawn, update_player_position_and_health))
        .add_systems(PostUpdate, render_player_move)
        // .add_systems(Update, test_render)
        ;
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct Player {
    x: u32,
    y: u32,
}

#[derive(Component, Debug, Copy, Clone)]
struct Health {
    value: u8,
}

#[derive(Component, Debug, Copy, Clone)]
struct RenderedPlayer;
const MULTIPLIER: f32 = 32.0;
const SCALE: Vec3 = Vec3::splat(1.3);

fn spawn_player(torii: Res<ToriiResource>, query: Query<&Player>, mut commands: Commands) {
    if query.iter().count() == 0 {
        let model = torii.data.models.iter().find_map(|model| {
            if model.name == "Player" {
                Some(model)
            } else {
                None
            }
        });

        let (x, y, health) = get_entity_data(model);
        match (x, y, health) {
            (x, y, Some(health)) => {
                info!("Spawning player at ({}, {}) with {} HP", x, y, health);
                commands.spawn((Player { x, y }, Health { value: health }));
            }
            (_, _, _) => {}
        }
    }
}

fn update_player_position_and_health(
    mut query: Query<(&mut Player, &mut Health)>,
    torii: Res<ToriiResource>,
) {
    for (mut player, mut health) in query.iter_mut() {
        let model = torii.data.models.iter().find_map(|model| {
            if model.name == "Player" {
                Some(model)
            } else {
                None
            }
        });

        let (x, y, hp) = get_entity_data(model);
        if x != player.x {
            info!(
                "Player moved from ({}, {}) to ({}, {})",
                player.x, player.y, x, y
            );
            player.x = x;
            player.y = y;
        }

        match hp {
            Some(hp) => {
                if hp != health.value {
                    info!("Player damaged. HP dropped from {} to {}", health.value, hp);
                    health.value = hp;
                }
            }
            None => {}
        }
    }
}

fn render_player_spawn(
    mut commands: Commands,
    query: Query<&Player>,
    query2: Query<&RenderedPlayer>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let num_rendered = query2.iter().count();
    for player in query.iter() {
        if num_rendered == 0 {
            let texture: Handle<Image> = asset_server.load("boy.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 4, 7, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            // commands.spawn(RenderedPlayer);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        ((player.x + 1) as f32 - 1.) * MULTIPLIER,
                        ((player.y + 1) as f32 - 1.) * MULTIPLIER,
                        // 0 as f32 * MULTIPLIER,
                        // 0 as f32 * MULTIPLIER,
                        1.0,
                    ))
                    .with_scale(SCALE),
                    texture: texture.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 3,
                },
                RenderedPlayer,
            ));
        }
    }
}

fn render_player_move(
    query: Query<&Player>,
    mut query2: Query<&mut Transform, With<RenderedPlayer>>,
) {
    for mut transform in query2.iter_mut() {
        for player in query.iter() {
            transform.translation = Vec3::new(
                ((player.x + 1) as f32 - 1.) * MULTIPLIER,
                ((player.y + 1) as f32 - 1.) * MULTIPLIER,
                1.0,
            );
        }
    }
}

fn test_render(
    mut commands: Commands,
    query2: Query<&RenderedPlayer>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if query2.iter().count() == 0 {
        let texture: Handle<Image> = asset_server.load("boy.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 4, 7, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.spawn(RenderedPlayer);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    ((0 + 1) as f32 - 1.) * MULTIPLIER,
                    ((1 + 1) as f32 - 1.) * MULTIPLIER,
                    // 0 as f32 * MULTIPLIER,
                    // 0 as f32 * MULTIPLIER,
                    1.0,
                ))
                .with_scale(SCALE),
                texture: texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 3,
            },
        ));
    }
}

fn get_entity_data(model: Option<&Model>) -> (u32, u32, Option<u8>) {
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

        let health = model.members[2].ty.as_primitive().unwrap().as_u8().unwrap();

        (x, y, Some(health))
    } else {
        (0, 0, None)
    }
}
