此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# HELPER

这个文件是 `crates/helper` 的 AI 规则。

`helper` 是跨 runtime / 跨 crate 的共享基础设施层。

它不代表某一个运行个体。它不属于 Bevy App，也不属于 `external_runtime`。

## 核心职责

- 提供跨世界通信、channel、transport 等基础设施。
- 未来可以放 network、序列化、协议辅助、进程间通信等共享 helper。
- 不保存游戏状态。

## 代码落点

- 通用 channel：写到 `crates/helper/src/channel.rs`。
- 未来 network helper：写到 `crates/helper/src/network`。

## 边界规则

- 不依赖 `gameplay`。
- 不依赖 `external_runtime`。
- 不依赖 `audio`、`ecs`、`physics`、`prefab`、`intent`、`render_2d`、`render_3d`。
- 不定义 gameplay、prefab、state 或 intent 消息语义。
- 不生成实体，不读取或修改 Bevy `World`。

## 依赖规则

- `helper` 必须依赖 `error`。
- `helper` 当前可以依赖 `bevy`，用于给通用 channel wrapper 实现 `Resource`。

## 验证要求

修改 `crates/helper` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
