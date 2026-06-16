use bevy::prelude::*;

use super::demo::demo_effect_lifetime_system;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_effect_lifetime_system);
    }
}
