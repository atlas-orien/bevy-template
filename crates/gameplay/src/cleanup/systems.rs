use bevy::prelude::*;
use prefab::ui::DemoMenuRootMarker;

pub fn despawn_demo_menu_system(
    mut commands: Commands,
    menu_roots: Query<Entity, With<DemoMenuRootMarker>>,
) {
    for menu_root in &menu_roots {
        commands.entity(menu_root).despawn();
    }
}
