use tokio::sync::watch;
use tokio::task::JoinHandle;

use crate::input::local::LocalKeyboardInput;
use crate::manager::ExternalRuntimeManager;
use crate::manager::set_movement_intent;

use prefab::identity::GameplayEntityId;

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
    let mut sources = ExternalSources::default();

    loop {
        tokio::select! {
            _ = interval.tick() => {
                manager.sync_gameplay_updates();
                sources.poll(&manager).await;
            }
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    break;
                }
            }
        }
    }
}

#[derive(Default)]
struct ExternalSources {
    local_keyboard: LocalKeyboardInput,
}

impl ExternalSources {
    async fn poll(&mut self, manager: &ExternalRuntimeManager) {
        self.poll_local_keyboard(manager);
    }

    fn poll_local_keyboard(&self, manager: &ExternalRuntimeManager) {
        let target = self.local_keyboard.movement_target();
        let _ = set_movement_intent(manager, GameplayEntityId(1), target);
    }
}
