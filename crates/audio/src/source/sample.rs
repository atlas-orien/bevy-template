#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AudioSampleSource {
    pub path: String,
}

impl AudioSampleSource {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}
