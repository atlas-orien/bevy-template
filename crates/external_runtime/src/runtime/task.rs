use tokio::sync::watch;
use tokio::task::JoinHandle;

use gameplay::api::RuntimeUpdateMessage;

use crate::input::ai::AiControlSource;
use crate::input::network::{NetworkSource, NetworkSourceConfig};
use crate::manager::ExternalRuntimeManager;

#[derive(Debug, Clone, Copy)]
pub struct ExternalRuntimeConfig {
    pub tick_interval: core::time::Duration,
    pub network: Option<NetworkSourceConfig>,
}

impl Default for ExternalRuntimeConfig {
    fn default() -> Self {
        Self {
            tick_interval: core::time::Duration::from_millis(16),
            network: None,
        }
    }
}

pub struct ExternalRuntime {
    shutdown: watch::Sender<bool>,
    task: JoinHandle<()>,
}

impl ExternalRuntime {
    pub fn spawn(config: ExternalRuntimeConfig, manager: ExternalRuntimeManager) -> Self {
        let (shutdown, shutdown_rx) = watch::channel(false);
        let task = tokio::spawn(run_external_runtime_loop(config, manager, shutdown_rx));

        Self { shutdown, task }
    }

    pub async fn shutdown(self) {
        let _ = self.shutdown.send(true);
        let _ = self.task.await;
    }
}

async fn run_external_runtime_loop(
    config: ExternalRuntimeConfig,
    manager: ExternalRuntimeManager,
    mut shutdown: watch::Receiver<bool>,
) {
    let mut interval = tokio::time::interval(config.tick_interval);
    let mut sources = ExternalSources::new(config).await;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let updates = manager.drain_gameplay_updates();
                sources.poll(&manager, updates).await;
            }
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    break;
                }
            }
        }
    }
}

struct ExternalSources {
    ai_control: AiControlSource,
    network: Option<NetworkSource>,
}

impl ExternalSources {
    async fn new(config: ExternalRuntimeConfig) -> Self {
        Self {
            ai_control: AiControlSource::default(),
            network: NetworkSource::connect(config.network).await,
        }
    }

    async fn poll(&mut self, manager: &ExternalRuntimeManager, updates: Vec<RuntimeUpdateMessage>) {
        self.ai_control.poll(manager);

        for update in updates {
            match update {
                RuntimeUpdateMessage::DemoNetworkLoginRequested => {
                    if let Some(network) = &mut self.network {
                        network.send_demo_login_request();
                    } else {
                        println!("network login test skipped: network source is disabled");
                    }
                }
                other => manager.apply_gameplay_update(other),
            }
        }

        if let Some(network) = &mut self.network {
            network.poll(manager).await;
        }
    }
}
