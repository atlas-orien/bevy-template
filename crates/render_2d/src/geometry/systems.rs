use bevy::prelude::*;

use super::RenderSize2d;

type RenderSize2dChanged = Or<(Changed<RenderSize2d>, Added<Sprite>)>;
type RenderSize2dQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static RenderSize2d, &'static mut Sprite),
    RenderSize2dChanged,
>;

pub fn sync_render_size_2d_system(mut sprites: RenderSize2dQuery) {
    for (size, mut sprite) in &mut sprites {
        sprite.custom_size = Some(size.0);
    }
}
