#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ProceduralAudioKind {
    Engine,
    Wind,
    Noise,
    Tone,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ProceduralAudioSource {
    pub kind: ProceduralAudioKind,
}
