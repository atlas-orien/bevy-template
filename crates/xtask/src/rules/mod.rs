mod app;
mod ecs;
mod error;
mod gameplay;
mod input;
mod intent;
mod physics;
mod prefab;
mod render_2d;

pub enum CheckStatus {
    Passed,
    Failed(Vec<String>),
}

pub fn check_architecture() -> CheckStatus {
    let mut errors = Vec::new();

    if let CheckStatus::Failed(mut rule_errors) = app::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = intent::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = input::check() {
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

    if let CheckStatus::Failed(mut rule_errors) = prefab::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = render_2d::check() {
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
