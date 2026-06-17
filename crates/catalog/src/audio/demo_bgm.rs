use prefab::world_2d::demo_level::DemoBgmAudio;

use crate::paths::DEMO_BGM_AUDIO;

pub struct DemoBgm;

impl DemoBgm {
    pub fn resource() -> DemoBgmAudio {
        DemoBgmAudio::new(DEMO_BGM_AUDIO)
    }
}
