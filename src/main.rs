use external_runtime::runtime::{ExternalRuntime, ExternalRuntimeConfig};
use gameplay::api::GameplayManager;

#[tokio::main]
async fn main() {
    let (gameplay_manager, gameplay_inbox) = GameplayManager::new();
    let external_runtime =
        ExternalRuntime::spawn(ExternalRuntimeConfig::default(), gameplay_manager);

    app::run(gameplay_inbox);
    external_runtime.shutdown().await;
}
