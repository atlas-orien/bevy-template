use cmdproto::{Cmd, M1001Tos};

use crate::protocol::NetworkPayload;

pub fn login(
    seq: u32,
    username: impl Into<String>,
    password: impl Into<String>,
) -> cmdproto::Result<NetworkPayload> {
    let message = M1001Tos {
        username: username.into(),
        password: password.into(),
    };

    Ok(NetworkPayload::new(cmdproto::encode_tos(
        Cmd::Cmd1001 as u32,
        seq,
        &message,
    )?))
}
