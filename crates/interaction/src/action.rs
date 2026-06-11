use bevy::prelude::*;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct InteractionAction {
    pub id: String,
}

impl InteractionAction {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
