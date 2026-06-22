use super::{animation_3d, capabilities, demo_game, demo_menu, primitives_3d, skeletal_animation};

pub const DEFAULT_PREVIEW: &str = "demo_game";

pub async fn run(name: &str) {
    match name {
        "animation_3d" => animation_3d::run(),
        "capabilities" => capabilities::run(),
        "demo_game" => demo_game::run().await,
        "demo_menu" => demo_menu::run().await,
        "primitives_3d" => primitives_3d::run(),
        "skeletal_animation" => skeletal_animation::run(),
        unknown => {
            eprintln!(
                "unknown dev preview `{unknown}`; available previews: demo_game, demo_menu, capabilities, primitives_3d, animation_3d, skeletal_animation"
            );
            std::process::exit(2);
        }
    }
}
