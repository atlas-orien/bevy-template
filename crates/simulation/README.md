# simulation

`simulation` 是游戏流程和世界调度层。

它不定义实体数据，不封装物理引擎，不读取输入，也不负责渲染。它负责把游戏状态、阶段、scene 进入/退出和 ECS 系统调度组织起来。

## 职责

- 定义游戏状态流，例如 Loading、MainMenu、Playing、Paused、GameOver。
- 定义不同状态进入、运行、退出时发生什么。
- 决定什么时候进入或退出 scene。
- 决定什么时候清理 scene entity。
- 决定在某个状态或阶段启用哪些 ECS system。
- 保持 app 很薄，让 app 只负责最终插件组装。

## 推荐结构

- `state`: 游戏状态定义和状态切换。
- `schedule`: 系统集合、运行条件、调度顺序。
- `spawning`: scene 进入调度，例如进入 Playing 时加载当前游戏场景。
- `cleanup`: 清理策略，例如退出 Playing 时清理 scene entity。
- `lifecycle`: 关卡、回合、场景等更高层生命周期。

当前代码里的旧模块可以逐步迁移到这个结构。目录可以按项目需求调整，但必须保持职责清楚。

## 和 ecs 的区别

`ecs` 定义数据和底层 ECS 系统函数。

`simulation` 决定这些系统在什么状态、什么阶段、什么顺序运行。

例如：

- `crates/ecs/src/systems/movement`: 定义 `movement_system`。
- `crates/simulation`: 决定 `movement_system` 只在 Playing 状态运行。

## 和 scenes/prefab 的区别

`prefab` 定义“一个对象由哪些组件组成”。

`scenes` 定义“一个场景由哪些 prefab 组成”。

`simulation` 定义“什么时候进入或退出场景”。

例如：

- `prefab`: 定义对象模板
- `scenes`: 定义场景包含哪些对象模板
- `simulation`: `OnEnter(AppState::Playing)` 时加载当前场景

## 和 intent 的区别

`intent` 表达 Entity 想做什么。

`simulation` 不关心意图来自哪里，只负责游戏流程和系统调度。

## 不应该放这里

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- 不封装 Avian、Rapier 或其它物理后端。
- 不直接读取键盘、手柄、鼠标、网络输入。
- 不加载精灵、模型、音频。
- 不播放动画。
- 不写 UI、相机、材质、灯光。
- 不在生成系统里散装大量组件，应该调用 `scenes`。
