# EXTERNAL_RUNTIME

这个文件是 `crates/external_runtime` 的 AI 规则。

`external_runtime` 是 Bevy App 外部的 runtime 和 manager-side adapter 层。

项目运行时有两套系统：

- Bevy App：运行 Bevy `World`、`Schedule`、render、physics、gameplay。
- External Runtime：运行 Bevy App 外部的输入、外设、AI、脚本、回放，以及未来网络等外部系统。

`external_runtime` 通过 `gameplay::api::GameplayManager` 向 Bevy App 提交 gameplay 请求。

## 核心职责

- 启动和停止 Bevy App 外部的 runtime loop。
- 持有 `GameplayManager`，作为外部系统进入 gameplay 的唯一入口。
- 管理 local/device/AI/script/replay 等外部来源模块。
- 把外部来源转换成 manager API 调用。
- 不直接读取或修改 Bevy `World`。

## 代码落点

- runtime loop：写到 `crates/external_runtime/src/runtime`。
- Bevy App 外部到 gameplay 的桥接：写到 `crates/external_runtime/src/bridge`。
- 本地输入来源：写到 `crates/external_runtime/src/local`。
- 外设来源：写到 `crates/external_runtime/src/device`。
- AI 控制来源：写到 `crates/external_runtime/src/ai`。

网络不是 v1 的子模块。网络是双向通信层，v2 单独设计。

## Runtime 规则

- `external_runtime::runtime` 是 Bevy App 外部的 loop/runner。
- 可以使用 Tokio 或其它异步运行机制。
- 不直接读取或修改 Bevy `World`。
- 不直接使用 `Commands`、`Query` 或 `ResMut`。
- 不做成 Bevy `Plugin` 注册到 `App` 里。
- 必须通过 `GameplayManager` 或明确的 bridge/manager API 进入 gameplay。

## Manager 规则

- 用户和外部模块优先通过 manager API 操作 gameplay。
- `GameplayRequest` 是 manager 到 gameplay 的内部请求，不应该被普通用户代码到处构造。
- manager API 不向用户暴露 Bevy `Entity`。
- manager 内部可以维护 gameplay-facing id 和 Bevy `Entity` 的映射。

## 边界规则

- 不定义核心 `Component`、`Bundle`、`Resource`、`Event`。
- 不生成实体。
- 不直接依赖或使用裸 `ecs`。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。
- 不定义或导出 Bevy `Plugin`。

## 依赖规则

- `external_runtime` 可以依赖 `gameplay`，用于持有 `GameplayManager`。
- `external_runtime` 可以依赖 `intent` 和 `prefab`，但优先通过 manager API。
- `external_runtime` 可以依赖 `tokio`。
- `external_runtime` 必须依赖 `error`。
- `external_runtime` 不依赖 `ecs`。
- `external_runtime` 不依赖 `physics`。
- `external_runtime` 不依赖 `render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/external_runtime` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
