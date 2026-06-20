use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct SkeletonId3d(pub u64);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct BoneId3d(pub u16);
