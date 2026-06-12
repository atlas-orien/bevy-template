// 此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

mod base;
mod crates;
mod util;

pub enum CheckStatus {
    Passed,
    Failed(Vec<String>),
}

pub fn finish(errors: Vec<String>) -> CheckStatus {
    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

pub fn check_architecture() -> CheckStatus {
    let mut errors = Vec::new();

    if let CheckStatus::Failed(mut rule_errors) = crates::app::check() {
        errors.append(&mut rule_errors);
    }

    if let CheckStatus::Failed(mut rule_errors) = crates::audio::check() {
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
