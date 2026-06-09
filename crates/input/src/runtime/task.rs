use tokio::sync::watch;
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy)]
pub struct InputRuntimeConfig {
    pub tick_interval: core::time::Duration,
}

impl Default for InputRuntimeConfig {
    fn default() -> Self {
        Self {
            tick_interval: core::time::Duration::from_millis(16),
        }
    }
}

pub struct InputRuntime {
    shutdown: watch::Sender<bool>,
    task: JoinHandle<()>,
}

impl InputRuntime {
    pub fn spawn(config: InputRuntimeConfig) -> Self {
        let (shutdown, shutdown_rx) = watch::channel(false);
        let task = tokio::spawn(run_input_loop(config, shutdown_rx));

        Self { shutdown, task }
    }

    pub async fn shutdown(self) {
        let _ = self.shutdown.send(true);
        let _ = self.task.await;
    }
}

async fn run_input_loop(config: InputRuntimeConfig, mut shutdown: watch::Receiver<bool>) {
    let mut interval = tokio::time::interval(config.tick_interval);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                poll_input_sources().await;
            }
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    break;
                }
            }
        }
    }
}

async fn poll_input_sources() {
    // Network, device, and AI input sources will be polled here.
}
