use cmdproto::M1001Toc;

use super::Input;

pub async fn handle_login(Input(data): Input<M1001Toc>) {
    println!(
        "network login reply: code={} token={} message={}",
        data.code, data.token, data.message
    );
}
