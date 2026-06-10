use bevy::prelude::*;

use crate::{
    NavigationPath2d, NavigationPath3d, NavigationTarget2d, NavigationTarget3d,
    StraightLineNavigationQuery2d, StraightLineNavigationQuery3d,
};

type RefreshPath2dFilter = Or<(Changed<NavigationTarget2d>, Added<NavigationPath2d>)>;
type RefreshPath3dFilter = Or<(Changed<NavigationTarget3d>, Added<NavigationPath3d>)>;

pub fn refresh_straight_line_paths_2d_system(
    mut agents: Query<
        (&Transform, &NavigationTarget2d, &mut NavigationPath2d),
        RefreshPath2dFilter,
    >,
) {
    let query = StraightLineNavigationQuery2d;

    for (transform, target, mut path) in &mut agents {
        let Some(target) = target.position() else {
            path.clear();
            continue;
        };

        *path = query.path(transform.translation.truncate(), target);
    }
}

pub fn refresh_straight_line_paths_3d_system(
    mut agents: Query<
        (&Transform, &NavigationTarget3d, &mut NavigationPath3d),
        RefreshPath3dFilter,
    >,
) {
    let query = StraightLineNavigationQuery3d;

    for (transform, target, mut path) in &mut agents {
        let Some(target) = target.position() else {
            path.clear();
            continue;
        };

        *path = query.path(transform.translation, target);
    }
}
