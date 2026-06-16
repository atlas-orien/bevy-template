use super::{capabilities, demo_menu, skeletal_animation};

pub const DEFAULT_PREVIEW: &str = "capabilities";

pub fn run(name: &str) {
    match name {
        "capabilities" => capabilities::run(),
        "demo_menu" => demo_menu::run(),
        "skeletal_animation" => skeletal_animation::run(),
        unknown => {
            eprintln!(
                "unknown dev preview `{unknown}`; available previews: capabilities, demo_menu, skeletal_animation"
            );
            std::process::exit(2);
        }
    }
}
