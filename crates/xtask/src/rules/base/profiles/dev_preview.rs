use crate::rules::base::derives::reject_derived_types;

pub struct DevPreviewRules<'a> {
    pub crate_path: &'a str,
}

pub fn check_dev_preview(rules: DevPreviewRules<'_>, errors: &mut Vec<String>) {
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event", "Message"],
        errors,
        "dev_preview should assemble previews from production crates instead of defining reusable ECS/gameplay types",
    );
}
