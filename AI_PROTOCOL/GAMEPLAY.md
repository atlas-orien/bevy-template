# GAMEPLAY

这个文件是 `crates/gameplay` 的 AI 规则。

`crates/gameplay` 是游戏玩法语义层，负责游戏流程和世界调度。

它不是 Bevy 底层 runtime；它是注册给 Bevy App/Schedule 执行的游戏状态流和 session 调度层。

## 核心职责

- 定义游戏状态流。
- 定义状态进入、运行、退出时的调度。
- 组装游戏玩法内部插件，例如 prefab、input、intent。
- 决定什么时候进入或退出 gameplay session。
- 决定什么时候清理 gameplay entity。
- 决定哪些 ECS system 在哪些状态或阶段运行。

## 代码落点

- 游戏状态：写到 `crates/gameplay/src/state`。
- 系统调度、运行条件、系统集合：写到 `crates/gameplay/src/schedule`。
- gameplay session 进入调度：写到 `crates/gameplay/src/spawning`。
- 清理策略：写到 `crates/gameplay/src/cleanup`。
- 关卡、回合、gameplay session 生命周期：写到 `crates/gameplay/src/lifecycle`。

当前旧目录可以逐步迁移，不需要保留旧名字。

## Spawning 目录规则

- `spawning/mod.rs` 只组装 `SpawningPlugin` 和模块导出。
- `spawning/plan.rs` 定义 gameplay spawn plan 数据结构。
- `spawning/prefab.rs` 定义 object-safe spawn item 抽象，并调用 `prefab` crate 的 `Prefab::spawn`。
- `spawning/defaults.rs` 定义模板默认 spawn plan。
- `spawning/systems.rs` 定义 Bevy spawning system。
- 不要把所有生成逻辑塞进 `spawning/mod.rs`。
- `GameplaySpawnPlan` 必须能接收任意实现 `Prefab` 的具体 prefab，不要维护中心 enum 或 match 列表。
- `gameplay` 负责决定何时执行 spawn plan；具体 prefab 内部组件组合仍然属于 `crates/prefab`。

## 边界规则

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- 不写底层 ECS 规则函数；需要调度底层规则时使用 `crates/prefab` 暴露的窄 facade。
- 不封装物理后端；这些放到 `crates/physics`。
- 不读取输入；输入来源放到 `crates/input`，再转换成 `crates/intent` 表达的意图。
- 不写渲染、动画、UI、相机；这些放到渲染层。
- 不直接散装实体组件；生成对象时优先调用 `crates/prefab`。

## 依赖规则

- `gameplay` 可以依赖 `prefab`，用于 gameplay setup 中使用封装好的对象模板、spawn API 和窄 facade。
- `gameplay` 可以依赖 `input` 和 `intent`，并作为唯一游戏玩法入口负责注册和调度它们。
- `gameplay` 必须依赖 `error`。
- `gameplay` 不依赖 `ecs`。
- `gameplay` 不依赖 `render_2d` 或 `render_3d`。
- `gameplay` 不直接依赖 `physics`；对象组合通过 `prefab` 完成，并由 `gameplay` 注册 `PrefabPlugin`。

## 验证要求

修改 `crates/gameplay` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
