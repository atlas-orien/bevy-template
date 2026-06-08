# simulation

`simulation` 是游戏 runtime 语义层，负责游戏流程和世界调度。

它不定义实体数据，不封装物理引擎，不读取输入，也不负责渲染。它负责把游戏状态、阶段、runtime session 进入/退出和 ECS 系统调度组织起来。

## 职责

- 定义游戏状态流，例如 Loading、MainMenu、Playing、Paused、GameOver。
- 定义不同状态进入、运行、退出时发生什么。
- 决定什么时候进入或退出 runtime session。
- 决定什么时候清理 runtime entity。
- 决定在某个状态或阶段启用哪些 ECS system。
- 保持 app 很薄，让 app 只负责最终插件组装。

## 推荐结构

- `state`: 游戏状态定义和状态切换。
- `schedule`: 系统集合、运行条件、调度顺序。
- `spawning`: runtime session 进入调度，例如进入 Playing 时加载当前游戏世界。
- `cleanup`: 清理策略，例如退出 Playing 时清理 runtime entity。
- `lifecycle`: 关卡、回合、runtime session 等更高层生命周期。

当前代码里的旧模块可以逐步迁移到这个结构。目录可以按项目需求调整，但必须保持职责清楚。

## 和 ecs 的区别

`ecs` 定义数据和底层 ECS 系统函数。

`prefab` 封装这些底层对象和规则，提供 runtime-facing 入口。

`simulation` 决定这些封装入口在什么状态、什么阶段、什么顺序运行。
`simulation` 也是游戏唯一 runtime，负责统一注册和调度 `prefab`、`input`、`intent`。

例如：

- `crates/ecs/src/systems/movement`: 定义 `movement_system`。
- `crates/prefab`: 封装 movement 等 runtime-facing 能力。
- `crates/simulation`: 决定 prefab runtime 能力只在 Playing 状态运行。

## 和 prefab 的区别

`prefab` 定义“一个对象由哪些组件组成”。

`simulation` 定义“当前 runtime session 使用哪些 prefab，以及什么时候生成或清理它们”。

例如：

- `prefab`: 定义对象模板
- `simulation`: `OnEnter(AppState::Playing)` 时加载当前游戏世界

## 和 intent 的区别

`intent` 表达 Entity 想做什么。

`simulation` 不关心意图来自哪里，只负责游戏流程和系统调度。

## 不应该放这里

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- 不直接依赖或调用 `ecs`。
- 不封装 Avian、Rapier 或其它物理后端。
- 不直接读取键盘、手柄、鼠标、网络输入。
- 不加载精灵、模型、音频。
- 不播放动画。
- 不写 UI、相机、材质、灯光。
- 不在生成系统里散装大量组件，应该调用 `prefab`。
