use bevy::prelude::*;

use super::{RenderColor2d, RenderOpacity2d, RenderVisibility2d};

type RenderColor2dChanged = Or<(
    Changed<RenderColor2d>,
    Changed<RenderOpacity2d>,
    Added<Sprite>,
)>;
type RenderColor2dQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static RenderColor2d,
        Option<&'static RenderOpacity2d>,
        &'static mut Sprite,
    ),
    RenderColor2dChanged,
>;

type RenderVisibility2dChanged = Or<(Changed<RenderVisibility2d>, Added<Visibility>)>;
type RenderVisibility2dQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static RenderVisibility2d, &'static mut Visibility),
    RenderVisibility2dChanged,
>;

pub fn sync_render_color_2d_system(mut sprites: RenderColor2dQuery) {
    for (color, opacity, mut sprite) in &mut sprites {
        sprite.color = opacity
            .map(|opacity| color.0.with_alpha(opacity.0.clamp(0.0, 1.0)))
            .unwrap_or(color.0);
    }
}

pub fn sync_render_visibility_2d_system(mut entities: RenderVisibility2dQuery) {
    for (render_visibility, mut visibility) in &mut entities {
        *visibility = if render_visibility.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
