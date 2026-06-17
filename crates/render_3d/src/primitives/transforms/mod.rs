use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LookAtTransform3d {
    translation: Vec3,
    target: Vec3,
    up: Vec3,
}

impl LookAtTransform3d {
    pub fn new(translation: Vec3, target: Vec3, up: Vec3) -> Self {
        Self {
            translation,
            target,
            up,
        }
    }

    pub fn into_transform(self) -> Transform {
        Transform::from_translation(self.translation).looking_at(self.target, self.up)
    }
}

impl From<LookAtTransform3d> for Transform {
    fn from(value: LookAtTransform3d) -> Self {
        value.into_transform()
    }
}
