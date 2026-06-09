use external_runtime::manager::ExternalRuntimeManager;
use external_runtime::manager::gameplay::GameplayBridgeApi;
use external_runtime::runtime::{ExternalRuntime, ExternalRuntimeConfig};
use gameplay::api::gameplay_request_channel;

#[tokio::main]
async fn main() {
    let (gameplay_requests, gameplay_inbox) = gameplay_request_channel();
    let gameplay_bridge_api = GameplayBridgeApi::new(gameplay_requests);
    let manager = ExternalRuntimeManager::new(gameplay_bridge_api);
    let external_runtime = ExternalRuntime::spawn(ExternalRuntimeConfig::default(), manager);

    app::run(gameplay_inbox);
    external_runtime.shutdown().await;
}
