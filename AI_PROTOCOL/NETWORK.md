此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# NETWORK

这个文件是 `crates/network` 的 AI 规则。

`network` 是前端游戏网络连接层，负责维护客户端到服务器的一条 UDP/MSRT 连接，并把 transport payload 暴露给外部 runtime。

它位于 `msrt-udp` 和 `external_runtime` 之间：

```text
network client
  -> external_runtime
  -> gameplay RuntimeRequestMessage
```

## 核心职责

- 持有前端客户端网络连接。
- 支持断线后自动重连。
- 定义连接状态和客户端事件。
- 定义 protobuf payload 边界。
- 保留 protobuf payload 边界；具体 `.proto` schema 确定后再接入编解码。
- 不直接进入 Bevy App、World 或 Schedule。

## 代码落点

- 客户端连接、断线、重连：写到 `crates/network/src/connection`。
- protobuf 或二进制 payload 边界：写到 `crates/network/src/protocol`。

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
- `network` 不做服务端 peer/session 管理；当前项目是前端框架，只维护一个服务器连接。
- 不新增 `session` 目录；如果未来需要多人服务端框架，单独设计。
- `network` 不写 gameplay 业务逻辑。

## 验证要求

修改 `crates/network` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
