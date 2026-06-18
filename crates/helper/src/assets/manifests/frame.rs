use std::{collections::BTreeMap, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::assets::ron;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrameManifest {
    pub image: String,
    pub frame_size: (u32, u32),
    pub columns: u32,
    pub rows: u32,
    pub clips: BTreeMap<String, FrameClipManifest>,
}

impl FrameManifest {
    pub fn from_path(path: impl AsRef<Path>) -> error::Result<Self> {
        let bytes = fs::read(path)?;
        ron::from_bytes(&bytes)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrameClipManifest {
    pub frames: Vec<usize>,
    pub fps: f32,
    pub repeat: bool,
}
