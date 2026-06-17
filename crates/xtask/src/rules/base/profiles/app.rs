use crate::rules::base::source::reject_terms_in_rust_files;

pub struct AppRules<'a> {
    pub crate_path: &'a str,
    pub forbidden_plugins: &'a [&'a str],
}

pub fn check_app(rules: AppRules<'_>, errors: &mut Vec<String>) {
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.forbidden_plugins,
        errors,
        "app should register gameplay and external adapter plugins only, so expose this through gameplay instead",
    );
}
