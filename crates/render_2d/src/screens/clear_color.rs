use bevy::prelude::*;

use crate::ui::theme;

pub fn setup_screen_clear_color_system(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = theme::BACKGROUND;
}
