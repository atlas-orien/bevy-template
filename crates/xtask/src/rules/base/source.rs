use std::path::Path;

use syn::visit::Visit;

use crate::rules::util::{parse_rust_file, rust_files};

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
    reject_type_paths_in_rust_files(root, terms, errors, hint);
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

pub fn reject_method_calls_with_tuple_arg(
    root: impl AsRef<Path>,
    method_names: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        let mut visitor = TupleMethodCallVisitor {
            method_names,
            hits: Vec::new(),
        };
        visitor.visit_file(&parsed);
        visitor.hits.sort();
        visitor.hits.dedup();

        for hit in visitor.hits {
            errors.push(format!(
                "{} calls `{hit}` with a tuple argument; {hint}",
                file.display()
            ));
        }
    }
}

pub fn reject_path_suffixes_in_rust_files(
    root: impl AsRef<Path>,
    forbidden_suffixes: &[&[&str]],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        let mut visitor = PathSuffixVisitor {
            forbidden_suffixes,
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

pub fn reject_string_literals_containing(
    root: impl AsRef<Path>,
    fragments: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        let mut visitor = StringLiteralVisitor {
            fragments,
            hits: Vec::new(),
        };
        visitor.visit_file(&parsed);
        visitor.hits.sort();
        visitor.hits.dedup();

        for hit in visitor.hits {
            errors.push(format!(
                "{} contains string literal `{hit}`; {hint}",
                file.display()
            ));
        }
    }
}

pub fn reject_public_fields_with_type(
    root: impl AsRef<Path>,
    forbidden_types: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        let mut visitor = PublicFieldTypeVisitor {
            forbidden_types,
            hits: Vec::new(),
        };
        visitor.visit_file(&parsed);
        visitor.hits.sort();
        visitor.hits.dedup();

        for hit in visitor.hits {
            errors.push(format!(
                "{} exposes public field of type `{hit}`; {hint}",
                file.display()
            ));
        }
    }
}

pub fn reject_direct_input_access(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_type_paths_in_rust_files(root, DIRECT_INPUT_TERMS, errors, hint);
}

pub fn reject_network_transport_terms(
    root: impl AsRef<Path>,
    errors: &mut Vec<String>,
    hint: &str,
) {
    reject_type_paths_in_rust_files(root, NETWORK_TRANSPORT_TERMS, errors, hint);
}

pub fn reject_world_mutation_terms(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_type_paths_in_rust_files(root, WORLD_MUTATION_TYPES, errors, hint);
}

pub fn reject_bevy_world_access(root: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    reject_type_paths_in_rust_files(root, BEVY_WORLD_ACCESS_TYPES, errors, hint);
}

struct ForbiddenTypePathVisitor<'a> {
    forbidden: &'a [&'a str],
    hits: Vec<String>,
}

struct TupleMethodCallVisitor<'a> {
    method_names: &'a [&'a str],
    hits: Vec<String>,
}

impl<'ast> Visit<'ast> for TupleMethodCallVisitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let method = node.method.to_string();
        if self.method_names.iter().any(|name| *name == method)
            && node.args.len() == 1
            && matches!(node.args.first(), Some(syn::Expr::Tuple(_)))
        {
            self.hits.push(method);
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

struct PathSuffixVisitor<'a> {
    forbidden_suffixes: &'a [&'a [&'a str]],
    hits: Vec<String>,
}

impl PathSuffixVisitor<'_> {
    fn check_path(&mut self, path: &syn::Path) {
        let segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>();

        for suffix in self.forbidden_suffixes {
            if segments.len() < suffix.len() {
                continue;
            }

            let start = segments.len() - suffix.len();
            if suffix
                .iter()
                .zip(&segments[start..])
                .all(|(expected, actual)| *expected == actual)
            {
                self.hits.push(suffix.join("::"));
            }
        }
    }
}

impl<'ast> Visit<'ast> for PathSuffixVisitor<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        self.check_path(path);
        syn::visit::visit_path(self, path);
    }
}

struct StringLiteralVisitor<'a> {
    fragments: &'a [&'a str],
    hits: Vec<String>,
}

impl<'ast> Visit<'ast> for StringLiteralVisitor<'_> {
    fn visit_lit_str(&mut self, node: &'ast syn::LitStr) {
        let value = node.value();
        if self
            .fragments
            .iter()
            .any(|fragment| value.contains(fragment))
        {
            self.hits.push(value);
        }
    }
}

struct PublicFieldTypeVisitor<'a> {
    forbidden_types: &'a [&'a str],
    hits: Vec<String>,
}

impl PublicFieldTypeVisitor<'_> {
    fn check_field(&mut self, field: &syn::Field) {
        if !matches!(field.vis, syn::Visibility::Public(_)) {
            return;
        }

        let syn::Type::Path(path) = &field.ty else {
            return;
        };

        for segment in &path.path.segments {
            let ident = segment.ident.to_string();
            if self
                .forbidden_types
                .iter()
                .any(|forbidden| *forbidden == ident)
            {
                self.hits.push(ident);
            }
        }
    }
}

impl<'ast> Visit<'ast> for PublicFieldTypeVisitor<'_> {
    fn visit_field(&mut self, node: &'ast syn::Field) {
        self.check_field(node);
        syn::visit::visit_field(self, node);
    }
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
    use super::{ForbiddenTypePathVisitor, PathSuffixVisitor};
    use syn::visit::Visit;

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

    fn path_suffix_hits(source: &str, forbidden_suffixes: &[&[&str]]) -> Vec<String> {
        let parsed = syn::parse_file(source).expect("test source should parse");
        let mut visitor = PathSuffixVisitor {
            forbidden_suffixes,
            hits: Vec::new(),
        };
        visitor.visit_file(&parsed);
        visitor.hits.sort();
        visitor.hits.dedup();
        visitor.hits
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

    #[test]
    fn path_suffixes_match_associated_items_without_text_scanning() {
        let source = r#"
            fn f() {
                let _ = TextureAtlasLayout::from_grid(size, 4, 4, None, None);
                let _ = bevy::render::render_resource::ImageArrayLayout::RowCount(4);
            }
        "#;

        let hits = path_suffix_hits(
            source,
            &[
                &["TextureAtlasLayout", "from_grid"],
                &["ImageArrayLayout", "RowCount"],
            ],
        );

        assert!(
            hits.iter()
                .any(|hit| hit == "TextureAtlasLayout::from_grid")
        );
        assert!(hits.iter().any(|hit| hit == "ImageArrayLayout::RowCount"));
    }
}
