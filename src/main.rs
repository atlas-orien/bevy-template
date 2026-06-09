use external_runtime::manager::ExternalRuntimeManager;
use external_runtime::runtime::{ExternalRuntime, ExternalRuntimeConfig};
use gameplay::api::{ManagerUpdateChannel, RuntimeRequestChannel};

#[tokio::main]
async fn main() {
    let runtime_requests = RuntimeRequestChannel::new();
    let manager_updates = ManagerUpdateChannel::new();

    let manager = ExternalRuntimeManager::new(runtime_requests.sender(), manager_updates.inbox());
    let external_runtime =
        ExternalRuntime::spawn(ExternalRuntimeConfig::default(), manager.clone());

    app::run(runtime_requests.inbox(), manager_updates.sender());
    external_runtime.shutdown().await;
}
