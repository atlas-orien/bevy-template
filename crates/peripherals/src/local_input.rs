//! Shared local input semantics for keyboard, mouse, and gamepad adapters.

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum LocalInputContext {
    #[default]
    UiNavigation,
    Gameplay,
    TextEntry,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LocalInputAction {
    UiPrevious,
    UiNext,
    UiLeft,
    UiRight,
    UiActivate,
    UiCancel,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Interact,
    AttackPrimary,
    Pause,
}
