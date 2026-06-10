use bevy::prelude::*;

use crate::Render2dPlugin;
use crate::characters::{Character2dRender, Character2dRenderBundle};

#[test]
fn character_bundle_contains_configured_bevy_sprite() {
    let mut app = App::new();
    app.init_resource::<ClearColor>();
    app.add_plugins(Render2dPlugin);

    let entity = app
        .world_mut()
        .spawn(Character2dRenderBundle::new(
            Color::srgb(0.2, 0.4, 0.6),
            Vec2::new(32.0, 48.0),
        ))
        .id();

    app.update();

    let entity_ref = app.world().entity(entity);
    let sprite = entity_ref.get::<Sprite>().unwrap();

    assert!(entity_ref.contains::<Character2dRender>());
    assert_eq!(sprite.color, Color::srgb(0.2, 0.4, 0.6));
    assert_eq!(sprite.custom_size, Some(Vec2::new(32.0, 48.0)));
}
