use bevy::prelude::*;

use super::RenderFlip2d;

type RenderFlip2dChanged = Or<(Changed<RenderFlip2d>, Added<Sprite>)>;
type RenderFlip2dQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static RenderFlip2d, &'static mut Sprite),
    RenderFlip2dChanged,
>;

pub fn sync_render_flip_2d_system(mut sprites: RenderFlip2dQuery) {
    for (flip, mut sprite) in &mut sprites {
        sprite.flip_x = flip.x;
        sprite.flip_y = flip.y;
    }
}
