use bevy::prelude::*;
use components::characters::player::{Facing, MovementIntent, Player};
use simulation::flow::AppState;

const CHARACTER_ATLAS_PATH: &str = "images/characters/bevy/gabe/gabe-idle-run.png";
const FRAME_SIZE: UVec2 = UVec2::splat(24);
const FRAME_COLUMNS: u32 = 7;
const FRAME_ROWS: u32 = 1;
const IDLE_FRAME: usize = 0;
const RUN_FIRST_FRAME: usize = 1;
const RUN_LAST_FRAME: usize = 6;
const ANIMATION_FRAME_SECONDS: f32 = 0.1;
const PLAYER_RENDER_SCALE: f32 = 6.0;

#[derive(Component)]
struct PlayerSprite;

#[derive(Component, Deref, DerefMut)]
struct PlayerAnimation(Timer);

pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            attach_player_sprite.run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            animate_player_sprite.run_if(in_state(AppState::Playing)),
        );
    }
}

fn attach_player_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut players: Query<(Entity, &mut Transform), (With<Player>, Without<PlayerSprite>)>,
) {
    let texture = asset_server.load(CHARACTER_ATLAS_PATH);
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        FRAME_SIZE,
        FRAME_COLUMNS,
        FRAME_ROWS,
        None,
        None,
    ));

    for (entity, mut transform) in &mut players {
        transform.scale = Vec3::splat(PLAYER_RENDER_SCALE);

        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: layout.clone(),
                    index: IDLE_FRAME,
                },
            ),
            PlayerSprite,
            PlayerAnimation(Timer::from_seconds(
                ANIMATION_FRAME_SECONDS,
                TimerMode::Repeating,
            )),
        ));
    }
}

fn animate_player_sprite(
    time: Res<Time>,
    mut players: Query<(&MovementIntent, &Facing, &mut PlayerAnimation, &mut Sprite)>,
) {
    for (movement_intent, facing, mut animation, mut sprite) in &mut players {
        sprite.flip_x = *facing == Facing::Left;

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        if movement_intent.direction == Vec2::ZERO {
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
