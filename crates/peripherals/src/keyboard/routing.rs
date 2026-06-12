use interaction::UiNavigationInputKind;

use crate::LocalInputAction;

pub fn ui_navigation_kind_for_action(action: LocalInputAction) -> Option<UiNavigationInputKind> {
    match action {
        LocalInputAction::UiPrevious => Some(UiNavigationInputKind::Previous),
        LocalInputAction::UiNext => Some(UiNavigationInputKind::Next),
        LocalInputAction::UiActivate => Some(UiNavigationInputKind::Activate),
        _ => None,
    }
}
