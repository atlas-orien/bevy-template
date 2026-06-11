use std::path::Path;

use crate::rules::util::{require_mod_rs_in_subdirs, require_path};

pub fn require_crate_anchor(crate_path: &str, protocol_path: &str, errors: &mut Vec<String>) {
    require_path(
        crate_path,
        errors,
        "crate architecture anchor must remain present",
    );
    require_path(
        protocol_path,
        errors,
        "AI protocol for this crate must remain present",
    );
}

pub fn require_mod_rs_under_src(crate_path: &str, errors: &mut Vec<String>) {
    require_mod_rs_in_subdirs(Path::new(crate_path).join("src"), errors);
}
