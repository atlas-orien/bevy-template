mod app;
mod audio;
mod ecs;
mod error;
mod external_runtime;
mod gameplay;
mod helper;
mod intent;
mod interaction;
mod navigation;
mod peripherals;
mod physics;
mod prefab;
mod render_2d;
mod render_3d;
mod util;

pub enum CheckStatus {
    Passed,
    Failed(Vec<String>),
}

pub fn check_architecture() -> CheckStatus {
    let mut errors = Vec::new();

    if let CheckStatus::Failed(mut rule_errors) = app::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = audio::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = intent::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = external_runtime::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = helper::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = interaction::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = ecs::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = error::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = physics::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = navigation::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = peripherals::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = prefab::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = render_2d::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = render_3d::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = gameplay::check() {
        errors.append(&mut rule_errors);
    }

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}
