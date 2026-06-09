# RUNTIME

这个文件是 `crates/runtime` 的 AI 规则。

`crates/runtime` 是游戏 runtime 语义层，负责游戏流程和世界调度。

## 核心职责

- 定义游戏状态流。
- 定义状态进入、运行、退出时的调度。
- 组装游戏 runtime 内部插件，例如 prefab、input、intent。
- 决定什么时候进入或退出 runtime session。
- 决定什么时候清理 runtime entity。
- 决定哪些 ECS system 在哪些状态或阶段运行。

## 代码落点

- 游戏状态：写到 `crates/runtime/src/state`。
- 系统调度、运行条件、系统集合：写到 `crates/runtime/src/schedule`。
- runtime/session 进入调度：写到 `crates/runtime/src/spawning`。
- 清理策略：写到 `crates/runtime/src/cleanup`。
- 关卡、回合、runtime session 生命周期：写到 `crates/runtime/src/lifecycle`。

当前旧目录可以逐步迁移，不需要保留旧名字。

## 边界规则

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- 不写底层 ECS 规则函数；需要调度底层规则时使用 `crates/prefab` 暴露的窄 facade。
- 不封装物理后端；这些放到 `crates/physics`。
- 不读取输入；输入来源放到 `crates/input`，再转换成 `crates/intent` 表达的意图。
- 不写渲染、动画、UI、相机；这些放到渲染层。
- 不直接散装实体组件；生成对象时优先调用 `crates/prefab`。

## 依赖规则

- `runtime` 可以依赖 `prefab`，用于 runtime setup 中使用封装好的对象模板、spawn API 和窄 facade。
- `runtime` 可以依赖 `input` 和 `intent`，并作为唯一游戏 runtime 负责注册和调度它们。
- `runtime` 必须依赖 `error`。
- `runtime` 不依赖 `ecs`。
- `runtime` 不依赖 `render_2d` 或 `render_3d`。
- `runtime` 不直接依赖 `physics`；对象组合通过 `prefab` 完成，并由 `runtime` 注册 `PrefabPlugin`。

## 验证要求

修改 `crates/runtime` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
