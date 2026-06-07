use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ErrorLog {
    pub entries: Vec<String>,
}
