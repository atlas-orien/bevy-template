use external_runtime::manager::ExternalRuntimeManager;
use external_runtime::runtime::{ExternalRuntime, ExternalRuntimeConfig};

#[tokio::main]
async fn main() {
    let manager = ExternalRuntimeManager::new();
    let gameplay_inbox = manager.gameplay_request_inbox();
    let gameplay_updates = manager.gameplay_update_sender();
    let external_runtime =
        ExternalRuntime::spawn(ExternalRuntimeConfig::default(), manager.clone());

    app::run(gameplay_inbox, gameplay_updates);
    external_runtime.shutdown().await;
}
