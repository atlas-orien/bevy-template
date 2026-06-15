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
- 使用 `cmdproto` 解包 cmd + protobuf payload。
- 使用 `fnroute` 分发已解码 protobuf 消息给 handler。
- 不直接进入 Bevy App、World 或 Schedule。

## 代码落点

- 模板 demo 协议依赖 crates.io 上的 `cmdproto = "0.1.0"`。
  - 这个依赖用于模板自测和 demo network button。
  - 脚手架生成真实项目后，把依赖替换成真实项目根目录下的 `../cmdproto`。
  - 前端代码只通过 crate 名 `cmdproto` 使用协议，不假设协议 crate 的物理路径。
- 客户端连接、断线、重连：写到 `crates/network/src/connection`。
- protobuf 或二进制 payload 边界：写到 `crates/network/src/protocol`。
- 发给服务端的 protobuf request 构造：写到 `crates/network/src/request`。
  - request 模块只负责把明确的 ToServer protobuf 消息编码成 `NetworkPayload`。
  - UI/gameplay 不直接调用 request；由 `external_runtime` 的 network source 调用。
- cmdproto + fnroute handler 桥接：写到 `crates/network/src/router`。
- 用户 protobuf handler 函数的落点：写到 `crates/network/src/handler`。
  - 这个模块导出 `fnroute::Input`。
  - 用户业务 handler 使用 `async fn handle_login(Input(data): Input<M1001Toc>)`。
  - handler 模块只放 handler 函数，不做注册。
  - cmd 到 handler 的注册和分发写在 `crates/network/src/router`。

## 边界规则

- `network` 可以依赖 `msrt-udp`。
- `network` 可以依赖 `cmdproto`，用于 cmd 包头和 protobuf payload 编解码。
- 在前端模板仓库中，`cmdproto` 必须使用 crates.io 版本，不能指向开发者本机的 `../cmdproto`。
- `network` 可以依赖 `fnroute`，用于通用 handler 函数分发。
- `network` 可以依赖 `prost`，用于 protobuf `Message` 约束。
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
- handler 参数使用 `fnroute::Input<T>`，例如 `async fn handle_login(Input(data): Input<M1001Toc>)`。

## 验证要求

修改 `crates/network` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
