use bevy::prelude::*;

use super::RenderZIndex2d;

type RenderZIndex2dChanged = Or<(Changed<RenderZIndex2d>, Added<Transform>)>;
type RenderZIndex2dQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static RenderZIndex2d, &'static mut Transform),
    RenderZIndex2dChanged,
>;

pub fn sync_render_z_index_2d_system(mut entities: RenderZIndex2dQuery) {
    for (z_index, mut transform) in &mut entities {
        transform.translation.z = z_index.0 as f32;
    }
}
