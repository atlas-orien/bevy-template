use bevy::prelude::*;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct DisplayName(pub String);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PublicEntityId(pub u64);

impl DisplayName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}
