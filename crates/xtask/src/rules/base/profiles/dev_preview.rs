use std::path::Path;

use crate::rules::base::dependencies::reject_workspace_manifest_terms_except;
use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::{reject_files_under_dir_except, require_mod_rs_under_src};
use crate::rules::base::source::{
    reject_lines_containing_all_terms, reject_terms_in_file, require_file_contains_all_terms,
};
use crate::rules::util::require_path;

pub struct DevPreviewRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub required_previews: &'a [&'a str],
}

pub fn check_dev_preview(rules: DevPreviewRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "dev_preview is the code-only development preview crate and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/DEV_PREVIEW.md documents the development preview rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/main.rs"),
        errors,
        "dev_preview needs a runnable main.rs entry point",
    );
    let previews_dir = Path::new(rules.crate_path).join("src/previews");
    require_path(
        &previews_dir,
        errors,
        "dev previews must live under crates/dev_preview/src/previews",
    );
    require_path(
        previews_dir.join("mod.rs"),
        errors,
        "dev_preview/src/previews/mod.rs should declare and dispatch preview scenes",
    );
    for preview in rules.required_previews {
        require_path(
            previews_dir.join(preview),
            errors,
            "dev_preview should keep required framework preview scenes available",
        );
    }
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_workspace_manifest_terms_except(
        "crates",
        rules.crate_path,
        &["dev_preview.workspace = true"],
        errors,
        "dev_preview is a top-level development tool; production crates must not depend on it",
    );
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event", "Message"],
        errors,
        "dev_preview should assemble previews from production crates instead of defining reusable ECS/gameplay types",
    );
    reject_lines_containing_all_terms(
        Path::new(rules.crate_path).join("src/main.rs"),
        &["App::new", ".run"],
        errors,
        "dev_preview main.rs should only parse/select a preview; put concrete Bevy preview setup in src/previews",
    );
    reject_terms_in_file(
        previews_dir.join("mod.rs"),
        &["App::new", ".add_plugins", ".add_systems", "Commands"],
        errors,
        "dev_preview previews/mod.rs should only declare modules and dispatch named previews",
    );
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/main.rs"),
        &["previews::DEFAULT_PREVIEW", "previews::run"],
        errors,
        "dev_preview main.rs should delegate preview selection to the previews module",
    );
    reject_files_under_dir_except(
        Path::new(rules.crate_path).join("src"),
        &["main.rs"],
        errors,
        "dev_preview root src directory should stay thin; put preview scenes under src/previews",
    );
}
