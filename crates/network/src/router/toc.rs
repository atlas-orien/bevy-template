use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use cmdproto::{Cmd, PacketToc};
use fnroute::Input;
use prost::Message;

use super::{NetworkRouteError, NetworkRouteResult};
use crate::handler::handle_login;

type BoxedTocFuture = Pin<Box<dyn Future<Output = NetworkRouteResult<()>> + Send>>;
type BoxedTocHandler = Box<dyn Fn(Vec<u8>) -> BoxedTocFuture + Send + Sync>;

#[derive(Default)]
pub struct TocRouter {
    handlers: HashMap<u32, BoxedTocHandler>,
}

impl TocRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn route<T, H, Fut>(mut self, cmd: u32, handler: H) -> Self
    where
        T: Message + Default + Clone + Send + Sync + 'static,
        H: Fn(Input<T>) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.handlers.insert(
            cmd,
            Box::new(move |payload| {
                let handler = handler.clone();
                Box::pin(async move {
                    let message = cmdproto::decode_payload::<T>(&payload)?;
                    handler(Input(message)).await;
                    Ok(())
                })
            }),
        );
        self
    }

    pub async fn dispatch_bytes(&self, bytes: &[u8]) -> NetworkRouteResult<()> {
        let packet = cmdproto::decode_packet_toc(bytes)?;
        self.dispatch_packet(packet).await
    }

    pub async fn dispatch_packet(&self, packet: PacketToc) -> NetworkRouteResult<()> {
        let Some(handler) = self.handlers.get(&packet.cmd) else {
            return Err(NetworkRouteError::UnknownCmd(packet.cmd));
        };

        handler(packet.payload).await
    }
}

pub fn demo_toc_router() -> TocRouter {
    TocRouter::new().route(Cmd::Cmd1001 as u32, handle_login)
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use cmdproto::{Cmd, M1001Toc};

    use super::*;

    async fn handle_login(Input(data): Input<M1001Toc>, captured: Arc<Mutex<Option<String>>>) {
        *captured.lock().unwrap() = Some(data.token);
    }

    #[tokio::test]
    async fn dispatches_decoded_toc_payload_to_input_handler() {
        let captured = Arc::new(Mutex::new(None));
        let router = TocRouter::new().route(Cmd::Cmd1001 as u32, {
            let captured = Arc::clone(&captured);
            move |input| handle_login(input, Arc::clone(&captured))
        });

        let message = M1001Toc {
            code: 0,
            token: "token-1".to_string(),
            message: "ok".to_string(),
        };
        let bytes = cmdproto::encode_toc(Cmd::Cmd1001 as u32, 1, 0, &message).unwrap();

        router.dispatch_bytes(&bytes).await.unwrap();

        assert_eq!(captured.lock().unwrap().as_deref(), Some("token-1"));
    }
}
