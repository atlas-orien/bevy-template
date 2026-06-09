use bevy::prelude::*;

use super::RuntimeRequest;

pub fn submit_gameplay_request(
    requests: &mut MessageWriter<RuntimeRequest>,
    request: RuntimeRequest,
) {
    requests.write(request);
}
