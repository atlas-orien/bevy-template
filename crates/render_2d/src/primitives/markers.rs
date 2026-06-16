use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct SceneCamera2d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FollowCameraTarget2d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct AtlasSprite2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct RenderLayer2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct TilemapChunkLayer2dMarker;
