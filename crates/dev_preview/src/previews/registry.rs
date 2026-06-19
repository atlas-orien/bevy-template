use super::{capabilities, demo_game, demo_menu, primitives_3d, skeletal_animation};

pub const DEFAULT_PREVIEW: &str = "demo_game";

pub fn run(name: &str) {
    match name {
        "capabilities" => capabilities::run(),
        "demo_game" => demo_game::run(),
        "demo_menu" => demo_menu::run(),
        "primitives_3d" => primitives_3d::run(),
        "skeletal_animation" => skeletal_animation::run(),
        unknown => {
            eprintln!(
                "unknown dev preview `{unknown}`; available previews: demo_game, demo_menu, capabilities, primitives_3d, skeletal_animation"
            );
            std::process::exit(2);
        }
    }
}
