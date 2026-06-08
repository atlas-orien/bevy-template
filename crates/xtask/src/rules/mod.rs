mod ecs;

pub fn check_architecture() -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if let Err(mut rule_errors) = ecs::check() {
        errors.append(&mut rule_errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
