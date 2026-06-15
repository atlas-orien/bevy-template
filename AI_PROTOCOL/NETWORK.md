此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# NETWORK

这个文件是 `crates/network` 的 AI 规则。

`network` 是游戏网络协议层，负责把 UDP transport payload 转换成游戏外部网络消息。

它位于 `msrt-udp` 和 `external_runtime` 之间：

```text
msrt-udp
  -> network
  -> external_runtime
  -> gameplay RuntimeRequestMessage
```

## 核心职责

- 持有网络 transport 的项目级封装。
- 定义网络连接、session、peer、payload 的项目级类型。
- 定义从网络收到的 inbound message 和准备发出的 outbound message。
- 保留 protobuf payload 边界；具体 `.proto` schema 确定后再接入编解码。
- 不直接进入 Bevy App、World 或 Schedule。

## 代码落点

- transport adapter：写到 `crates/network/src/transport`。
- protobuf 或二进制 payload 边界：写到 `crates/network/src/protocol`。
- 网络 session、peer、connection id：写到 `crates/network/src/session`。

## 边界规则

- `network` 可以依赖 `msrt-udp`。
- `network` 可以依赖 `tokio`。
- `network` 必须依赖 `error`。
- `network` 不依赖 `bevy`。
- `network` 不依赖 `gameplay`。
- `network` 不依赖 `external_runtime`。
- `network` 不依赖 `ecs`、`prefab`、`intent`、`physics`、`render_2d`、`render_3d`。
- `network` 不定义 Bevy `Component`、`Bundle`、`Resource`、`Event` 或 `Message`。
- `network` 不构造 `RuntimeRequestMessage`。
- `network` 不知道 `RuntimeUserId` 或 `RuntimeObjectId` 的 gameplay 含义；这些映射属于 `external_runtime` manager/bridge。
- `network` 不写 gameplay 业务逻辑。

## 验证要求

修改 `crates/network` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
