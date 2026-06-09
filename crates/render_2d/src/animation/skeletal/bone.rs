use bevy::prelude::*;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Bone2d {
    pub name: String,
}

impl Bone2d {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
