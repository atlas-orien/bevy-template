use std::path::Path;

use crate::rules::base::dependencies::{reject_dependencies, require_workspace_dependency};
use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::{require_mod_rs_under_src, require_paths};
use crate::rules::base::source::{
    reject_bevy_world_access, reject_direct_input_access, reject_terms_in_rust_files,
    require_file_contains_all_terms,
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
        "network needs a crate root that exports connection/protocol boundaries",
    );
    require_paths(
        rules.required_dirs,
        errors,
        "network must keep client connection and protocol boundaries explicit",
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
        "cmdproto",
        errors,
        "network should use cmdproto for cmd packet decode/encode",
    );
    require_workspace_dependency(
        rules.crate_path,
        "fnroute",
        errors,
        "network should use fnroute for user handler dispatch",
    );
    require_workspace_dependency(
        rules.crate_path,
        "prost",
        errors,
        "network should constrain routed protobuf messages with prost::Message",
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
        "network should stay a frontend connection/protocol layer and must not depend on gameplay or Bevy world crates",
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
        "network should expose client connection/protocol payloads only; external_runtime owns gameplay-facing id mapping",
    );
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/connection/client.rs"),
        &["msrt_udp", "UdpClient", "Reconnecting"],
        errors,
        "network connection should wrap msrt-udp client and own frontend reconnect state",
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
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/router/toc.rs"),
        &["cmdproto::decode_packet_toc", "fnroute::Input"],
        errors,
        "network router should bridge cmdproto packets into fnroute Input<T> handlers",
    );
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/request/tos.rs"),
        &["cmdproto::encode_tos", "M1001Tos", "NetworkPayload"],
        errors,
        "network request should construct explicit ToServer protobuf payloads without UI/gameplay dependencies",
    );
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/handler/mod.rs"),
        &["pub use fnroute::Input"],
        errors,
        "network handler module should re-export fnroute Input<T> for user handler functions",
    );
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/handler/toc.rs"),
        &["async fn", "Input<M1001Toc>"],
        errors,
        "network handler files should contain concrete protobuf handler functions, not generic registration",
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
