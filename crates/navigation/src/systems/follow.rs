use bevy::prelude::*;

use crate::{NavigationAgent2d, NavigationAgent3d, NavigationPath2d, NavigationPath3d};

pub fn follow_navigation_paths_2d_system(
    time: Res<Time>,
    mut agents: Query<(&NavigationAgent2d, &mut NavigationPath2d, &mut Transform)>,
) {
    for (agent, mut path, mut transform) in &mut agents {
        let Some(waypoint) = path.current_waypoint() else {
            continue;
        };

        let current = transform.translation.truncate();
        let offset = waypoint - current;
        let distance = offset.length();

        if distance <= agent.stopping_distance {
            path.advance();
            continue;
        }

        let step = agent.speed * time.delta_secs();
        let movement = offset.normalize_or_zero() * step.min(distance);
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

pub fn follow_navigation_paths_3d_system(
    time: Res<Time>,
    mut agents: Query<(&NavigationAgent3d, &mut NavigationPath3d, &mut Transform)>,
) {
    for (agent, mut path, mut transform) in &mut agents {
        let Some(waypoint) = path.current_waypoint() else {
            continue;
        };

        let current = transform.translation;
        let offset = waypoint - current;
        let distance = offset.length();

        if distance <= agent.stopping_distance {
            path.advance();
            continue;
        }

        let step = agent.speed * time.delta_secs();
        let movement = offset.normalize_or_zero() * step.min(distance);
        transform.translation += movement;
    }
}
