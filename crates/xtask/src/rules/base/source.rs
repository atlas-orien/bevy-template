use std::path::Path;

use syn::visit::Visit;

use crate::rules::util::{parse_rust_file, read_file_if_exists, rust_files};

pub const DIRECT_INPUT_TERMS: &[&str] = &["ButtonInput", "KeyCode", "MouseButton", "Gamepad"];

pub const NETWORK_TRANSPORT_TERMS: &[&str] = &[
    "protobuf",
    "prost",
    "socket",
    "TcpStream",
    "UdpSocket",
    "WebSocket",
];

pub const WORLD_MUTATION_TYPES: &[&str] = &[
    "Commands",
    "Query",
    "Transform",
    "PhysicsBody",
    "PhysicsCollider",
];

pub const BEVY_WORLD_ACCESS_TYPES: &[&str] = &["World", "Commands", "Query", "Res", "ResMut"];

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

/// 检查 Rust 文件中是否引用了禁止类型。基于语法树而不是子串：
/// use alias、use tree、类型位置和完全限定路径都会命中。
///
/// 已知残余盲区：宏调用体内部的 token 不展开检查；glob import 本身不报错，
/// 但 glob import 后裸用禁止类型会由路径检查命中。
pub fn reject_type_paths_in_rust_files(
    root: impl AsRef<Path>,
    forbidden: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        let mut visitor = ForbiddenTypePathVisitor {
            forbidden,
            hits: Vec::new(),
        };
        visitor.visit_file(&parsed);
        visitor.hits.sort();
        visitor.hits.dedup();

        for hit in visitor.hits {
            errors.push(format!("{} references `{hit}`; {hint}", file.display()));
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
    reject_type_paths_in_rust_files(root, DIRECT_INPUT_TERMS, errors, hint);
}

pub fn reject_network_transport_terms(
    root: impl AsRef<Path>,
    errors: &mut Vec<String>,
    hint: &str,
) {
    reject_terms_in_rust_files(root, NETWORK_TRANSPORT_TERMS, errors, hint);
}

pub fn reject_world_mutation_terms(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_type_paths_in_rust_files(root, WORLD_MUTATION_TYPES, errors, hint);
}

pub fn reject_bevy_world_access(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_type_paths_in_rust_files(root, BEVY_WORLD_ACCESS_TYPES, errors, hint);
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

struct ForbiddenTypePathVisitor<'a> {
    forbidden: &'a [&'a str],
    hits: Vec<String>,
}

impl ForbiddenTypePathVisitor<'_> {
    fn check_ident(&mut self, ident: &syn::Ident) {
        let ident = ident.to_string();
        if self.forbidden.iter().any(|forbidden| *forbidden == ident) {
            self.hits.push(ident);
        }
    }

    fn check_path(&mut self, path: &syn::Path) {
        for segment in &path.segments {
            self.check_ident(&segment.ident);
        }
    }
}

impl<'ast> Visit<'ast> for ForbiddenTypePathVisitor<'_> {
    fn visit_use_tree(&mut self, tree: &'ast syn::UseTree) {
        match tree {
            syn::UseTree::Path(path) => {
                self.check_ident(&path.ident);
                self.visit_use_tree(&path.tree);
            }
            syn::UseTree::Name(name) => self.check_ident(&name.ident),
            syn::UseTree::Rename(rename) => self.check_ident(&rename.ident),
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    self.visit_use_tree(item);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        self.check_path(path);
        syn::visit::visit_path(self, path);
    }
}

#[cfg(test)]
mod tests {
    use super::ForbiddenTypePathVisitor;
    use syn::visit::Visit;

    fn contains_any_term(source: &str, terms: &[&str]) -> bool {
        terms.iter().any(|term| source.contains(term))
    }

    fn line_contains_all_terms(source: &str, terms: &[&str]) -> bool {
        source
            .lines()
            .map(str::trim)
            .any(|line| terms.iter().all(|term| line.contains(term)))
    }

    fn forbidden_type_hits(source: &str, forbidden: &[&str]) -> Vec<String> {
        let parsed = syn::parse_file(source).expect("test source should parse");
        let mut visitor = ForbiddenTypePathVisitor {
            forbidden,
            hits: Vec::new(),
        };
        visitor.visit_file(&parsed);
        visitor.hits.sort();
        visitor.hits.dedup();
        visitor.hits
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

    #[test]
    fn detects_forbidden_type_aliases_and_qualified_paths() {
        let source = r#"
            use bevy::prelude::Transform as Xform;
            use bevy::prelude::{Commands as C, Entity};
            fn f(q: Query<&mut bevy::transform::components::Transform>) {}
            type T = bevy::prelude::Transform;
        "#;

        let hits = forbidden_type_hits(source, &["Transform", "Commands", "Query"]);

        assert!(hits.iter().any(|hit| hit == "Transform"));
        assert!(hits.iter().any(|hit| hit == "Commands"));
        assert!(hits.iter().any(|hit| hit == "Query"));
    }

    #[test]
    fn forbidden_type_paths_do_not_match_comments_strings_or_prefixes() {
        let source = r#"
            /// 把 Transform 留给 ecs 层处理。
            pub struct TransformRequest;
            fn f() {
                let s = "Transform";
            }
            mod transform_rules {}
        "#;

        let hits = forbidden_type_hits(source, &["Transform"]);

        assert!(hits.is_empty());
    }
}
