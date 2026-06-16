use bevy::prelude::*;
use prefab::ui::DemoMenuRoot;

pub fn despawn_demo_menu_system(
    mut commands: Commands,
    menu_roots: Query<Entity, With<DemoMenuRoot>>,
) {
    for menu_root in &menu_roots {
        commands.entity(menu_root).despawn();
    }
}
