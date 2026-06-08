use bevy::prelude::*;
use ecs::components::characters::player::{Facing, MovementIntent};

const CHARACTER_ATLAS_PATH: &str = "images/characters/bevy/gabe/gabe-idle-run.png";
const FRAME_SIZE: UVec2 = UVec2::splat(24);
const FRAME_COLUMNS: u32 = 7;
const FRAME_ROWS: u32 = 1;
const IDLE_FRAME: usize = 0;
const RUN_FIRST_FRAME: usize = 1;
const RUN_LAST_FRAME: usize = 6;
const ANIMATION_FRAME_SECONDS: f32 = 0.1;
const PLAYER_RENDER_SCALE: f32 = 6.0;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PlayerSprite;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerAnimation(pub Timer);

pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_player_sprite);
    }
}

#[derive(Bundle)]
pub struct PlayerSpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub marker: PlayerSprite,
    pub animation: PlayerAnimation,
}

impl PlayerSpriteBundle {
    pub fn from_assets(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let texture = asset_server.load(CHARACTER_ATLAS_PATH);
        let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            FRAME_SIZE,
            FRAME_COLUMNS,
            FRAME_ROWS,
            None,
            None,
        ));

        Self {
            sprite: Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout,
                    index: IDLE_FRAME,
                },
            ),
            transform: Transform::from_scale(Vec3::splat(PLAYER_RENDER_SCALE)),
            marker: PlayerSprite,
            animation: PlayerAnimation(Timer::from_seconds(
                ANIMATION_FRAME_SECONDS,
                TimerMode::Repeating,
            )),
        }
    }
}

fn animate_player_sprite(
    time: Res<Time>,
    parents: Query<&ChildOf, With<PlayerSprite>>,
    gameplay_entities: Query<(&MovementIntent, &Facing)>,
    mut sprites: Query<(Entity, &mut PlayerAnimation, &mut Sprite), With<PlayerSprite>>,
) {
    for (entity, mut animation, mut sprite) in &mut sprites {
        let Ok(parent) = parents.get(entity) else {
            continue;
        };
        let Ok((movement_intent, facing)) = gameplay_entities.get(parent.parent()) else {
            continue;
        };

        sprite.flip_x = *facing == Facing::Left;

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        if !movement_intent.is_moving() {
            animation.reset();
            atlas.index = IDLE_FRAME;
            continue;
        }

        animation.tick(time.delta());

        if animation.just_finished() {
            atlas.index = if atlas.index >= RUN_LAST_FRAME {
                RUN_FIRST_FRAME
            } else {
                (atlas.index + 1).max(RUN_FIRST_FRAME)
            };
        }
    }
}
