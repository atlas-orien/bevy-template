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
