use super::{demo_menu, skeletal_animation};

pub const DEFAULT_PREVIEW: &str = "skeletal_animation";

pub fn run(name: &str) {
    match name {
        "demo_menu" => demo_menu::run(),
        "skeletal_animation" => skeletal_animation::run(),
        unknown => {
            eprintln!(
                "unknown dev preview `{unknown}`; available previews: demo_menu, skeletal_animation"
            );
            std::process::exit(2);
        }
    }
}
