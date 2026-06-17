use std::path::Path;

use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::require_mod_rs_under_src;
use crate::rules::base::source::{
    reject_bevy_world_access, reject_direct_input_access, reject_terms_in_rust_files,
};

pub struct NetworkRules<'a> {
    pub crate_path: &'a str,
}

pub fn check_network(rules: NetworkRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event", "Message"],
        errors,
        "network should not define Bevy ECS data or gameplay messages",
    );
    reject_bevy_world_access(
        rules.crate_path,
        errors,
        "network must not access Bevy World; external_runtime bridges network payloads into gameplay",
    );
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "network should not read local peripherals",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        &["RuntimeRequestMessage", "RuntimeUserId", "RuntimeObjectId"],
        errors,
        "network should expose client connection/protocol payloads only; external_runtime owns gameplay-facing id mapping",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        &[
            "UdpServer",
            "NetworkSessionId",
            "NetworkPeerId",
            "src/session",
        ],
        errors,
        "network is a frontend client layer; do not add server peer/session management here",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path)
            .join("src/handler")
            .to_string_lossy()
            .as_ref(),
        &["route_toc", "TocRouter", "HashMap", "Input<T>"],
        errors,
        "network handler should not register routes or use generic Input<T>; router owns cmd-to-handler registration",
    );
}
