use gameplay::api::{GameplayRequest, GameplayRequestSender};

#[derive(Clone)]
pub struct GameplayBridgeApi {
    requests: GameplayRequestSender,
}

impl GameplayBridgeApi {
    pub fn new(requests: GameplayRequestSender) -> Self {
        Self { requests }
    }

    pub fn submit(&self, request: GameplayRequest) -> bool {
        self.requests.submit(request)
    }
}
