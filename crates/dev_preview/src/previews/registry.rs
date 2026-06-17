use super::{capabilities, demo_menu, primitives_3d, skeletal_animation};

pub const DEFAULT_PREVIEW: &str = "capabilities";

pub fn run(name: &str) {
    match name {
        "capabilities" => capabilities::run(),
        "demo_menu" => demo_menu::run(),
        "primitives_3d" => primitives_3d::run(),
        "skeletal_animation" => skeletal_animation::run(),
        unknown => {
            eprintln!(
                "unknown dev preview `{unknown}`; available previews: capabilities, demo_menu, primitives_3d, skeletal_animation"
            );
            std::process::exit(2);
        }
    }
}
