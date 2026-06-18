use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::assets::ron;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TilesetSourceConfig {
    pub rows: u32,
    pub tile_size: (u32, u32),
}

impl TilesetSourceConfig {
    pub fn from_bytes(bytes: &[u8]) -> error::Result<Self> {
        ron::from_bytes(bytes)
    }

    pub fn from_path(path: impl AsRef<Path>) -> error::Result<Self> {
        let bytes = fs::read(path)?;
        Self::from_bytes(&bytes)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TilesetManifest {
    pub image: String,
    pub array_rows: u32,
    pub tile_size: (u32, u32),
}

impl TilesetManifest {
    pub fn from_bytes(bytes: &[u8]) -> error::Result<Self> {
        ron::from_bytes(bytes)
    }

    pub fn from_path(path: impl AsRef<Path>) -> error::Result<Self> {
        let bytes = fs::read(path)?;
        Self::from_bytes(&bytes)
    }
}
