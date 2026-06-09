use bevy::prelude::*;

use super::request::GameplayRequest;

pub fn submit_gameplay_request(
    requests: &mut MessageWriter<GameplayRequest>,
    request: GameplayRequest,
) {
    requests.write(request);
}
