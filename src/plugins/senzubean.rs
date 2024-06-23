use super::torii::ToriiResource;
use bevy::prelude::*;
use torii_grpc::types::schema::Model;

pub struct SenzubeanPlugin;
impl Plugin for SenzubeanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_senzubean, render_arena));
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct Senzubean {
    x: u32,
    y: u32,
    is_eaten: bool,
}

#[derive(Component, Debug, Copy, Clone)]
struct RenderedSenzubean;
const MULTIPLIER: f32 = 32.0;
const SCALE: Vec3 = Vec3::splat(0.3);

fn spawn_senzubean(torii: Res<ToriiResource>, query: Query<&Senzubean>, mut commands: Commands) {
    if query.iter().count() == 0 {
        let model = torii.data.models.iter().find_map(|model| {
            if model.name == "Senzubean" {
                Some(model)
            } else {
                None
            }
        });

        let (x, y, is_eaten) = get_entity_data(model);
        match (x, y, is_eaten) {
            (x, y, Some(is_eaten)) => {
                info!("Spawning senzu bean at ({}, {})", x, y);
                commands.spawn(Senzubean { x, y, is_eaten });
            }
            (_, _, _) => {}
        }
    }
}

fn render_arena(
    mut commands: Commands,
    query: Query<&Senzubean>,
    query2: Query<&RenderedSenzubean>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for senzubean in query.iter() {
        if query2.iter().count() == 0 {
            let texture: Handle<Image> = asset_server.load("bean.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(64., 64.), 6, 15, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            commands.spawn(RenderedSenzubean);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        (senzubean.x as f32 - 1.) * MULTIPLIER,
                        (senzubean.y as f32 - 1.) * MULTIPLIER,
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
                    index: 89,
                },
            ));
        }
    }
}

fn get_entity_data(model: Option<&Model>) -> (u32, u32, Option<bool>) {
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

        let is_eaten = model.members[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_bool()
            .unwrap();

        (x, y, Some(is_eaten))
    } else {
        (0, 0, None)
    }
}
