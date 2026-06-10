use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct NavigationAgent2d {
    pub speed: f32,
    pub stopping_distance: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct NavigationAgent3d {
    pub speed: f32,
    pub stopping_distance: f32,
}

impl Default for NavigationAgent2d {
    fn default() -> Self {
        Self {
            speed: 180.0,
            stopping_distance: 1.0,
        }
    }
}

impl Default for NavigationAgent3d {
    fn default() -> Self {
        Self {
            speed: 4.0,
            stopping_distance: 0.05,
        }
    }
}
