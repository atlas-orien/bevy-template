pub mod enter;
pub mod exit;
pub mod sets;
pub mod update;

use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        sets::register_schedule_sets(app);
        enter::register_enter_schedules(app);
        update::register_update_schedules(app);
        exit::register_exit_schedules(app);
    }
}
