//! User-editable 2D presentation content, cameras, UI, and render systems.

pub mod animation;
pub mod atlases;
pub mod background;
pub mod camera;
pub mod characters;
pub mod debug;
pub mod effects;
pub mod environment;
pub mod images;
pub mod items;
pub mod layers;
pub mod lighting;
pub mod materials;
pub mod mesh;
pub mod overlays;
pub mod particles;
pub mod pixel;
pub mod props;
pub mod screens;
pub mod text;
pub mod tilemap;
pub mod transitions;
pub mod ui;

pub use error::Result;

use bevy::prelude::*;

use self::animation::Animation2dPlugin;
use self::atlases::AtlasesPlugin;
use self::background::BackgroundPlugin;
use self::camera::Camera2dPlugin;
use self::characters::CharacterRenderPlugin;
use self::debug::DebugRenderPlugin;
use self::effects::EffectsPlugin;
use self::environment::EnvironmentPlugin;
use self::items::ItemsPlugin;
use self::layers::Layers2dPlugin;
use self::lighting::Lighting2dPlugin;
use self::materials::Materials2dPlugin;
use self::mesh::Mesh2dContentPlugin;
use self::overlays::OverlaysPlugin;
use self::particles::ParticlesPlugin;
use self::pixel::PixelPlugin;
use self::props::PropsPlugin;
use self::screens::ScreensPlugin;
use self::text::Text2dContentPlugin;
use self::tilemap::TilemapPlugin;
use self::transitions::TransitionsPlugin;
use self::ui::UiPlugin;

pub struct Render2dPlugin;

impl Plugin for Render2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Camera2dPlugin)
            .add_plugins(AtlasesPlugin)
            .add_plugins(BackgroundPlugin)
            .add_plugins(CharacterRenderPlugin)
            .add_plugins(ItemsPlugin)
            .add_plugins(PropsPlugin)
            .add_plugins(TilemapPlugin)
            .add_plugins(Layers2dPlugin)
            .add_plugins(Lighting2dPlugin)
            .add_plugins(Materials2dPlugin)
            .add_plugins(Mesh2dContentPlugin)
            .add_plugins(EnvironmentPlugin)
            .add_plugins(EffectsPlugin)
            .add_plugins(ParticlesPlugin)
            .add_plugins(PixelPlugin)
            .add_plugins(Text2dContentPlugin)
            .add_plugins(OverlaysPlugin)
            .add_plugins(TransitionsPlugin)
            .add_plugins(DebugRenderPlugin)
            .add_plugins(Animation2dPlugin)
            .add_plugins(ScreensPlugin)
            .add_plugins(UiPlugin);
    }
}
