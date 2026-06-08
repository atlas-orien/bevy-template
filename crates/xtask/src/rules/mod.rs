mod ecs;
mod error;
mod input;
mod intent;
mod physics;
mod prefab;
mod render_2d;
mod scenes;
mod simulation;

pub enum CheckStatus {
    Passed,
    Failed(Vec<String>),
}

pub fn check_architecture() -> CheckStatus {
    let mut errors = Vec::new();

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

    if let CheckStatus::Failed(mut rule_errors) = scenes::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = simulation::check() {
        errors.append(&mut rule_errors);
    }

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}
