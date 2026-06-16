use super::base::assets::require_animated_2d_frame_manifests;
use super::base::readability::check_workspace_readability;
use super::{CheckStatus, crates};

pub fn check_architecture() -> CheckStatus {
    let mut errors = Vec::new();

    check_workspace_readability("crates", &mut errors);
    require_animated_2d_frame_manifests("assets/2d/animated", &mut errors);

    if let CheckStatus::Failed(mut rule_errors) = crates::app::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::audio::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::catalog::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::dev_preview::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::intent::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::external_runtime::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::helper::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::interaction::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::ecs::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::error::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::physics::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::navigation::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::network::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::peripherals::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::prefab::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::render_2d::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::render_3d::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::gameplay::check() {
        errors.append(&mut rule_errors);
    }

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}
