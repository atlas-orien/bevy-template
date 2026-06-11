use bevy::prelude::*;

use super::RuntimeRequestMessage;

pub fn submit_gameplay_request(
    requests: &mut MessageWriter<RuntimeRequestMessage>,
    request: RuntimeRequestMessage,
) {
    requests.write(request);
}
