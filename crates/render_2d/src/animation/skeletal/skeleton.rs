use bevy::prelude::*;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Skeleton2d {
    pub name: String,
}

impl Skeleton2d {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
