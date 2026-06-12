use super::demo_menu;

pub const DEFAULT_PREVIEW: &str = "demo_menu";

pub fn run(name: &str) {
    match name {
        "demo_menu" => demo_menu::run(),
        unknown => {
            eprintln!("unknown dev preview `{unknown}`; available previews: demo_menu");
            std::process::exit(2);
        }
    }
}
