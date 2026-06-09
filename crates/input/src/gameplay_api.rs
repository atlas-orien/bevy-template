use bevy::prelude::*;
use gameplay::api::{GameplayRequest, submit_gameplay_request};

pub fn submit_input_gameplay_request(
    requests: &mut MessageWriter<GameplayRequest>,
    request: GameplayRequest,
) {
    submit_gameplay_request(requests, request);
}
