use bevy::prelude::*;

use super::{Model3d, Model3dBundle};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ModelInstance3dMarker;

#[derive(Debug, Clone)]
pub struct ModelInstance3d {
    model: Model3d,
    transform: Transform,
    visibility: Visibility,
}

impl ModelInstance3d {
    pub fn new(model: Model3d, transform: Transform) -> Self {
        Self {
            model,
            transform,
            visibility: Visibility::default(),
        }
    }

    pub fn visible(model: Model3d, transform: Transform) -> Self {
        Self {
            model,
            transform,
            visibility: Visibility::Visible,
        }
    }

    pub fn hidden(model: Model3d, transform: Transform) -> Self {
        Self {
            model,
            transform,
            visibility: Visibility::Hidden,
        }
    }

    pub fn model(&self) -> &Model3d {
        &self.model
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn into_bundle(self) -> ModelInstance3dBundle {
        ModelInstance3dBundle {
            marker: ModelInstance3dMarker,
            model: self.model.into_bundle(),
            transform: self.transform,
            visibility: self.visibility,
        }
    }
}

#[derive(Bundle)]
pub struct ModelInstance3dBundle {
    marker: ModelInstance3dMarker,
    model: Model3dBundle,
    transform: Transform,
    visibility: Visibility,
}
