# gameplay

`gameplay` 是游戏玩法语义层，负责游戏流程和世界调度。

它不定义实体数据，不封装物理引擎，不读取输入，也不负责渲染。它负责把游戏状态、阶段、gameplay session 进入/退出和 ECS 系统调度组织起来。

它不是 Bevy 底层 runtime。Bevy 的 `App`、`World` 和 `Schedule` 负责真正执行系统；`gameplay` 只是把游戏玩法流程注册给 Bevy。

## 职责

- 定义游戏状态流，例如 Loading、MainMenu、Playing、Paused、GameOver。
- 定义不同状态进入、运行、退出时发生什么。
- 提供外部来源进入 gameplay 的 API 边界。
- 决定什么时候进入或退出 gameplay session。
- 决定什么时候清理 gameplay entity。
- 决定在某个状态或阶段启用哪些 ECS system。
- 保持 app 很薄，让 app 只负责最终插件组装。

## 推荐结构

- `state`: 游戏状态定义和状态切换。
- `api`: 外部来源进入 gameplay 的统一 API 边界。
- `schedule`: 系统集合、运行条件、调度顺序。
- `spawning`: gameplay session 进入调度，例如进入 Playing 时加载当前游戏世界。
- `cleanup`: 清理策略，例如退出 Playing 时清理 gameplay entity。
- `lifecycle`: 关卡、回合、gameplay session 等更高层生命周期。

当前代码里的旧模块可以逐步迁移到这个结构。目录可以按项目需求调整，但必须保持职责清楚。

## api 结构

`api` 是外部来源进入 gameplay 的统一边界。

它表达：

```text
外部或上层逻辑希望 gameplay 做什么。
```

它不直接执行：

```text
Commands
World mutation
Prefab::spawn
```

真正执行 API 请求的地方应该是 gameplay 内部 system，并注册到明确的 Bevy schedule。

适合走 API 的事情：

- 运行中生成或销毁对象。
- 切换 gameplay state。
- 加载关卡。
- 传送 Entity。
- 给予物品。
- 触发剧情或对话。

不一定适合走 API 的事情：

- 已有 Entity 的连续移动输入。
- 已有 Entity 的瞄准方向。
- 已有 Entity 的普通攻击意图。

这些更像 Entity 自己的 intent。

当前阶段 API 先放在 `crates/gameplay/src/api`。如果未来 input、network、script 等外部 crate 需要直接依赖这些类型，再考虑抽成独立 crate。

## spawning 结构

`spawning` 是 gameplay 管理 prefab 生成的标准目录。

- `mod.rs`: 只组装 `SpawningPlugin`。
- `plan.rs`: 定义“这次 gameplay session 要生成什么”。
- `prefab.rs`: 定义 object-safe spawn item 抽象。
- `defaults.rs`: 定义模板默认 spawn plan。
- `systems.rs`: 定义真正执行 spawn plan 的 Bevy system。

用户要改默认生成内容时，优先改 `defaults.rs`。新增具体 prefab 时，不应该维护中心 enum 或 match 列表；只要该 prefab 实现 `prefab::Prefab`，就可以进入 `GameplaySpawnPlan`。

## 和 ecs 的区别

`ecs` 定义数据和底层 ECS 系统函数。

`prefab` 封装这些底层对象和规则，提供 gameplay-facing 入口。

`gameplay` 决定这些封装入口在什么状态、什么阶段、什么顺序运行。
`gameplay` 也是游戏唯一玩法入口，负责统一注册和调度 `prefab`、`input`、`intent`。

例如：

- `crates/ecs/src/systems/movement`: 定义 `movement_system`。
- `crates/prefab`: 暴露 movement 等 gameplay-facing 窄 facade。
- `crates/gameplay`: 决定 prefab gameplay 能力只在 Playing 状态运行。

## 和 prefab 的区别

`prefab` 定义“一个对象由哪些组件组成”。

`gameplay` 定义“当前 gameplay session 使用哪些 prefab，以及什么时候生成或清理它们”。

例如：

- `prefab`: 定义对象模板
- `gameplay`: `OnEnter(AppState::Playing)` 时加载当前游戏世界

## 和 intent 的区别

`intent` 表达 Entity 想做什么。

`gameplay` 不关心意图来自哪里，只负责游戏流程和系统调度。

## 不应该放这里

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- 不直接依赖或调用 `ecs`。
- 不封装 Avian、Rapier 或其它物理后端。
- 不直接读取键盘、手柄、鼠标、网络输入。
- 不加载精灵、模型、音频。
- 不播放动画。
- 不写 UI、相机、材质、灯光。
- 不在生成系统里散装大量组件，应该调用 `prefab`。
