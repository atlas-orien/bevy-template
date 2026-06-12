use std::path::Path;

use crate::rules::util::{read_file_if_exists, rust_files};

pub const DIRECT_INPUT_TERMS: &[&str] = &["ButtonInput", "KeyCode", "MouseButton", "Gamepad"];

pub const NETWORK_TRANSPORT_TERMS: &[&str] = &[
    "protobuf",
    "prost",
    "socket",
    "TcpStream",
    "UdpSocket",
    "WebSocket",
];

pub const WORLD_MUTATION_TERMS: &[&str] = &[
    "Commands",
    "Query<(&mut Transform",
    "Query<&mut Transform",
    "Transform",
    "PhysicsBody",
    "PhysicsCollider",
];

pub const BEVY_WORLD_ACCESS_TERMS: &[&str] = &["World", "Commands", "Query<", "Res<", "ResMut<"];

pub fn reject_terms_in_rust_files(
    root: impl AsRef<Path>,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for term in terms {
            if source.contains(term) {
                errors.push(format!("{} references `{term}`; {hint}", file.display()));
            }
        }
    }
}

pub fn reject_files_containing_all_terms(
    root: impl AsRef<Path>,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        if terms.iter().all(|term| source.contains(term)) {
            errors.push(format!("{} {hint}", file.display()));
        }
    }
}

pub fn reject_lines_containing_all_terms(
    root: impl AsRef<Path>,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        if source
            .lines()
            .map(str::trim)
            .any(|line| terms.iter().all(|term| line.contains(term)))
        {
            errors.push(format!("{} {hint}", file.display()));
        }
    }
}

pub fn require_file_contains_all_terms(
    file: impl AsRef<Path>,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let file = file.as_ref();
    let Some(source) = read_file_if_exists(file) else {
        return;
    };

    for term in terms {
        if !source.contains(term) {
            errors.push(format!(
                "{} does not contain `{term}`; {hint}",
                file.display()
            ));
        }
    }
}

pub fn reject_terms_in_file(
    file: impl AsRef<Path>,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let file = file.as_ref();
    let Some(source) = read_file_if_exists(file) else {
        return;
    };

    for term in terms {
        if source.contains(term) {
            errors.push(format!("{} references `{term}`; {hint}", file.display()));
        }
    }
}

pub fn reject_generated_terms_in_file(
    file: impl AsRef<Path>,
    terms: &[&str],
    pattern: impl Fn(&str) -> Vec<String>,
    errors: &mut Vec<String>,
    hint: &str,
) {
    let file = file.as_ref();
    let Some(source) = read_file_if_exists(file) else {
        return;
    };

    for term in terms {
        for generated in pattern(term) {
            if source.contains(&generated) {
                errors.push(format!(
                    "{} references `{generated}`; {hint}",
                    file.display()
                ));
            }
        }
    }
}

pub fn reject_generated_terms_in_rust_files_except(
    root: impl AsRef<Path>,
    except_root: impl AsRef<Path>,
    terms: &[&str],
    pattern: impl Fn(&str) -> Vec<String>,
    errors: &mut Vec<String>,
    hint: &str,
) {
    let generated_terms = terms
        .iter()
        .flat_map(|term| pattern(term))
        .collect::<Vec<_>>();
    let generated_terms = generated_terms
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();

    reject_terms_in_rust_files_except(root, except_root, &generated_terms, errors, hint);
}

pub fn reject_direct_input_access(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_terms_in_rust_files(root, DIRECT_INPUT_TERMS, errors, hint);
}

pub fn reject_network_transport_terms(
    root: impl AsRef<Path>,
    errors: &mut Vec<String>,
    hint: &str,
) {
    reject_terms_in_rust_files(root, NETWORK_TRANSPORT_TERMS, errors, hint);
}

pub fn reject_world_mutation_terms(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_terms_in_rust_files(root, WORLD_MUTATION_TERMS, errors, hint);
}

pub fn reject_bevy_world_access(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_terms_in_rust_files(root, BEVY_WORLD_ACCESS_TERMS, errors, hint);
}

pub fn reject_terms_in_rust_files_except(
    root: impl AsRef<Path>,
    except_root: impl AsRef<Path>,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let except_root = except_root.as_ref();

    for file in rust_files(root) {
        if file.starts_with(except_root) {
            continue;
        }

        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for term in terms {
            if source.contains(term) {
                errors.push(format!("{} references `{term}`; {hint}", file.display()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn contains_any_term(source: &str, terms: &[&str]) -> bool {
        terms.iter().any(|term| source.contains(term))
    }

    fn line_contains_all_terms(source: &str, terms: &[&str]) -> bool {
        source
            .lines()
            .map(str::trim)
            .any(|line| terms.iter().all(|term| line.contains(term)))
    }

    #[test]
    fn detects_any_forbidden_term() {
        let source = "TextFont { font_size: 22.0, ..default() }";

        assert!(contains_any_term(source, &["TextFont", "TextColor"]));
    }

    #[test]
    fn detects_terms_on_same_line() {
        let source = "pub ui_camera: Entity,";

        assert!(line_contains_all_terms(source, &["pub ", ": Entity"]));
    }

    #[test]
    fn ignores_terms_split_across_lines() {
        let source = "pub struct Example;\nlet entity: Entity = todo!();";

        assert!(!line_contains_all_terms(source, &["pub ", ": Entity"]));
    }
}
