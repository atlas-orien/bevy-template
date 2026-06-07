use bevy::prelude::*;

use crate::{ErrorEvent, ErrorLog, ErrorSeverity};

pub struct ErrorPlugin;

impl Plugin for ErrorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ErrorEvent>()
            .init_resource::<ErrorLog>()
            .add_systems(Update, collect_errors);
    }
}

fn collect_errors(mut events: MessageReader<ErrorEvent>, mut log: ResMut<ErrorLog>) {
    for event in events.read() {
        let code = event.error.code();

        match event.error.severity() {
            ErrorSeverity::Info => info!(code = code, "{}", event.error),
            ErrorSeverity::Warning => warn!(code = code, "{}", event.error),
            ErrorSeverity::Recoverable => {
                error!(code = code, "{}", event.error);
            }
            ErrorSeverity::Fatal => {
                error!(code = code, "fatal: {}", event.error);
            }
        }

        log.entries.push(event.error.to_string());
    }
}
