use bevy::prelude::*;

use super::demo_particles::{
    demo_particle_emission_system, demo_particle_update_system, demo_player_dust_system,
    demo_sensor_particle_burst_system,
};

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                demo_player_dust_system,
                demo_sensor_particle_burst_system,
                demo_particle_emission_system,
                demo_particle_update_system,
            )
                .chain(),
        );
    }
}
