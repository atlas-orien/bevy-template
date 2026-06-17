use std::path::Path;

use crate::rules::util::{reject_path, require_mod_rs_in_subdirs};

pub fn require_mod_rs_under_src(crate_path: &str, errors: &mut Vec<String>) {
    require_mod_rs_in_subdirs(Path::new(crate_path).join("src"), errors);
}

pub fn reject_paths(paths: &[&str], errors: &mut Vec<String>, hint: &str) {
    for path in paths {
        reject_path(path, errors, hint);
    }
}
