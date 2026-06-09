use tokio::sync::watch;
use tokio::task::JoinHandle;

use crate::manager::ExternalRuntimeManager;

#[derive(Debug, Clone, Copy)]
pub struct ExternalRuntimeConfig {
    pub tick_interval: core::time::Duration,
}

impl Default for ExternalRuntimeConfig {
    fn default() -> Self {
        Self {
            tick_interval: core::time::Duration::from_millis(16),
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

    loop {
        tokio::select! {
            _ = interval.tick() => {
                manager.sync_gameplay_updates();
                poll_external_sources(&manager).await;
            }
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    break;
                }
            }
        }
    }
}

async fn poll_external_sources(_manager: &ExternalRuntimeManager) {
    // Local, device, and AI sources will be polled here.
}
