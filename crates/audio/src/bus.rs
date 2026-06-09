#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AudioBus {
    #[default]
    Master,
    Music,
    Sfx,
    Ui,
    Ambience,
}
