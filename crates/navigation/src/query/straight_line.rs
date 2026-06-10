use bevy::prelude::*;

use crate::{NavigationPath2d, NavigationPath3d};

#[derive(Debug, Clone, Copy, Default)]
pub struct StraightLineNavigationQuery2d;

#[derive(Debug, Clone, Copy, Default)]
pub struct StraightLineNavigationQuery3d;

impl StraightLineNavigationQuery2d {
    pub fn path(&self, _start: Vec2, end: Vec2) -> NavigationPath2d {
        NavigationPath2d::new(vec![end])
    }
}

impl StraightLineNavigationQuery3d {
    pub fn path(&self, _start: Vec3, end: Vec3) -> NavigationPath3d {
        NavigationPath3d::new(vec![end])
    }
}
