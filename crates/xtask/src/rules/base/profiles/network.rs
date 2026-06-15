use std::path::Path;

use crate::rules::base::dependencies::{reject_dependencies, require_workspace_dependency};
use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::{require_mod_rs_under_src, require_paths};
use crate::rules::base::source::{
    reject_bevy_world_access, reject_direct_input_access, reject_network_transport_terms,
    reject_terms_in_rust_files, require_file_contains_all_terms,
};
use crate::rules::util::require_path;

pub struct NetworkRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub required_dirs: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
}

pub fn check_network(rules: NetworkRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "network is the project network protocol crate and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/NETWORK.md documents the network boundary rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "network needs a crate root that exports protocol/session/transport boundaries",
    );
    require_paths(
        rules.required_dirs,
        errors,
        "network must keep transport, protocol, and session boundaries explicit",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    require_workspace_dependency(
        rules.crate_path,
        "msrt-udp",
        errors,
        "network should use msrt-udp as the UDP transport adapter",
    );
    require_workspace_dependency(
        rules.crate_path,
        "tokio",
        errors,
        "network should use Tokio-compatible async transport",
    );
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "network should stay a transport/protocol/session layer and must not depend on gameplay or Bevy world crates",
    );
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
        "network should expose transport/protocol payloads only; external_runtime owns gameplay-facing id mapping",
    );
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/transport/msrt_udp.rs"),
        &["msrt_udp::UdpClient", "msrt_udp::UdpServer"],
        errors,
        "network transport should wrap the published msrt-udp adapter",
    );
    reject_network_transport_terms(
        Path::new(rules.crate_path).join("src/protocol"),
        errors,
        "network protocol payload definitions should not own socket transport details",
    );
}
