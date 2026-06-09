use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Health(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct MaxHealth(pub f32);

impl Default for Health {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Default for MaxHealth {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Health {
    pub fn is_empty(self) -> bool {
        self.0 <= 0.0
    }
}
