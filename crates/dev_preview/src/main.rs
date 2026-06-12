mod previews;

fn main() {
    let preview = std::env::args()
        .nth(1)
        .unwrap_or_else(|| previews::DEFAULT_PREVIEW.to_string());

    previews::run(&preview);
}
