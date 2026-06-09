use input::runtime::{InputRuntime, InputRuntimeConfig};

#[tokio::main]
async fn main() {
    let input_runtime = InputRuntime::spawn(InputRuntimeConfig::default());
    app::run();
    input_runtime.shutdown().await;
}
