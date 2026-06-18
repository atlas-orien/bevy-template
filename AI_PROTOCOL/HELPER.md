此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# HELPER

这个文件是 `crates/helper` 的 AI 规则。

`helper` 是跨 runtime / 跨 crate 的共享基础设施层。

它不代表某一个运行个体。它不属于 Bevy App，也不属于 `external_runtime`。

## 核心职责

- 提供跨世界通信、channel、transport 等基础设施。
- 提供跨 crate 共享、无游戏语义的 Bevy 资源加载 helper，例如普通 image、渲染 texture 的 sRGB / linear 加载设置、shader handle 加载入口、薄 RON 反序列化工具和标准 runtime manifest schema。
- 未来可以放 network、序列化、协议辅助、进程间通信等共享 helper。
- 不保存游戏状态。

## 代码落点

- 通用 channel：写到 `crates/helper/src/channel.rs`。
- 通用 Bevy asset helper：写到 `crates/helper/src/assets`。
- runtime manifest schema 写到 `crates/helper/src/assets/manifests`；它定义 RON 文件格式和 `from_bytes` / `from_path` 读取入口，不注册 Bevy `AssetLoader`，不转换成 render/gameplay 类型。
- RON helper 只负责 `bytes -> T` 反序列化；具体 Bevy `AssetLoader` 和资源语义属于对应 render / gameplay 模块。
- 标准 manifest 的读取必须优先通过 `assets::manifests::*Manifest` 或对应 config 类型的 API；不要在业务 crate 里直接调用底层 RON helper 解析标准 manifest。
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
