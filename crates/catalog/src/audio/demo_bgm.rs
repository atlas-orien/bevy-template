use prefab::world_2d::demo_level::DemoBgmAudio;

const DEMO_BGM_AUDIO: &str = "audio/demo_bgm.ogg";

pub struct DemoBgm;

impl DemoBgm {
    pub fn resource() -> DemoBgmAudio {
        DemoBgmAudio::new(DEMO_BGM_AUDIO)
    }
}
