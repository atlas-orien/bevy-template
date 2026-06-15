use external_runtime::config::ExternalRuntimeSettings;
use external_runtime::manager::ExternalRuntimeManager;
use external_runtime::runtime::{ExternalRuntime, ExternalRuntimeConfig};
use gameplay::api::{ManagerUpdateChannel, RuntimeRequestChannel};

#[tokio::main]
async fn main() {
    let runtime_requests = RuntimeRequestChannel::new();
    let manager_updates = ManagerUpdateChannel::new();

    let manager = ExternalRuntimeManager::new(runtime_requests.sender(), manager_updates.inbox());
    let runtime_config = ExternalRuntimeSettings::load("config/services.toml")
        .map(ExternalRuntimeConfig::from)
        .unwrap_or_else(|error| {
            eprintln!("failed to load config/services.toml: {error}; using default runtime config");
            ExternalRuntimeConfig::default()
        });
    let external_runtime = ExternalRuntime::spawn(runtime_config, manager.clone());

    app::run(runtime_requests.inbox(), manager_updates.sender());
    external_runtime.shutdown().await;
}
