# GAMEPLAY

这个文件是 `crates/gameplay` 的 AI 规则。

`crates/gameplay` 是游戏玩法语义层，负责游戏流程和世界调度。

它不是 Bevy 底层 runtime；它是注册给 Bevy App/Schedule 执行的游戏状态流和 session 调度层。

## 核心职责

- 定义游戏状态流。
- 定义状态进入、运行、退出时的调度。
- 组装游戏玩法内部插件，例如 prefab、intent。
- 决定什么时候进入或退出 gameplay session。
- 决定什么时候清理 gameplay entity。
- 决定哪些 ECS system 在哪些状态或阶段运行。

## 代码落点

- 游戏状态：写到 `crates/gameplay/src/state`。
- 外部进入 gameplay 的 API 边界：写到 `crates/gameplay/src/api`。
- 系统调度、运行条件、系统集合：写到 `crates/gameplay/src/schedule`。
- gameplay session 进入调度：写到 `crates/gameplay/src/spawning`。
- 清理策略：写到 `crates/gameplay/src/cleanup`。
- 关卡、回合、gameplay session 生命周期：写到 `crates/gameplay/src/lifecycle`。
- UI 和世界对象被点击、hover 后的具体业务处理：写到 `crates/gameplay/src/interaction` 下的分类目录。
- UI 交互业务处理：写到 `crates/gameplay/src/interaction/ui`，demo 菜单逻辑写到 `ui/demo_menu.rs`。

当前旧目录可以逐步迁移，不需要保留旧名字。

## Spawning 目录规则

- `spawning` 是 gameplay 内部“生成流程”的标准落点，不只表示初始化生成。
- `spawning/mod.rs` 只做模块导出，不注册 Bevy schedule，不写具体 spawn 逻辑。
- `spawning/plan.rs` 定义 gameplay spawn plan 数据结构。
- `gameplay::api::SpawnItem` 定义 object-safe spawn item 抽象，并调用 `prefab` crate 的 `Prefab::spawn`。
- `spawning/initial.rs` 定义进入 gameplay session 时的默认 spawn plan 和对应 Bevy system。
- `spawning/runtime.rs` 定义运行中 spawn 的 gameplay 内部执行入口。
- 不要把所有生成逻辑塞进 `spawning/mod.rs`。
- spawn system 的 `OnEnter(...)` 注册写到 `schedule/enter.rs`。
- `GameplaySpawnPlan` 必须能接收任意实现 `Prefab` 的具体 prefab，不要维护中心 enum 或 match 列表。
- `gameplay` 负责决定何时执行 spawn plan；具体 prefab 内部组件组合仍然属于 `crates/prefab`。
- 初始化生成只写在 `spawning/initial.rs`。
- 运行中生成不要写在 `initial.rs`；由 API/request 消费逻辑调用 `spawning/runtime.rs` 中的窄入口。

## API 目录规则

- `api` 是外部来源进入 gameplay 的统一 API 边界。
- `api` 暴露外部可以提交的 gameplay 请求类型和提交函数。
- `api` 的消息类型和提交函数只表达“希望 gameplay 做什么”，不直接修改 Bevy `World`。
- `api` 的消息类型和提交函数不直接调用 `Commands`、`World` 或 `Prefab::spawn`。
- `api/systems.rs` 是 API 边界的 Bevy-side 消费执行层，可以使用 `Commands` 执行请求。
- object-safe spawn adapter 可以调用 `Prefab::spawn`，但只能作为 `RuntimeRequestMessage::SpawnPrefab` 的内部执行细节。
- `api` 不暴露 Bevy `Entity` 给外部来源；外部请求必须使用 gameplay-facing id。
- gameplay 内部负责把 gameplay-facing id 映射成 Bevy `Entity`。
- `api` 可以注册 Bevy `Message`，作为外部系统和 gameplay 内部系统之间的连接。
- `api` 的消费和执行必须放在 gameplay 内部 system 中，并注册到明确的 Bevy schedule。
- `api` 可以定义 request/update 消息语义，并使用 `helper` 的 channel 机制让 external runtime 和 Bevy App 双向通信。
- `RuntimeRequestMessage` / `RuntimeUpdateMessage` 的底层 transport 类型放在 `api/runtime_channel`。
- manager 不属于 `gameplay`，必须放在 `external_runtime`。
- gameplay 不依赖 manager，也不调用 manager；gameplay 只向 update channel 发消息。
- channel 机制属于 `helper`，不属于 `gameplay`。
- 用于 external runtime transport 的 `Resource` 只能放在 `api` 边界中，不表示 gameplay 内部状态数据。
- 运行中 spawn、despawn、状态切换、关卡加载、传送、给予物品等高层请求，都优先通过 API 进入 gameplay。
- 已有 Entity 的连续意图，例如移动、瞄准、攻击输入，不一定属于 API 请求；这类行为可以继续由 intent 层表达。
- 未来如果外部 crate 需要直接依赖 API 类型，再考虑把 API 抽成独立 crate；现在先放在 `crates/gameplay/src/api`。

当前最小 API 请求样板：

- `SpawnPrefab`: 运行中生成 prefab。
- `DespawnEntity`: 按 gameplay-facing id 销毁实体。
- `ClearSession`: 清理当前 gameplay session 生成的实体。
- `ChangeState`: 请求切换 gameplay state。
- `SetMovementIntent`: 按 gameplay-facing id 设置移动意图。

新增 runtime/world 请求时，消息类型放到 `api/runtime_channel/message.rs`，object-safe spawn adapter 放到 `api/runtime_channel/spawn.rs`，消费逻辑放到 `api/systems.rs`，不要散落到其它目录。

## Schedule 目录规则

- `schedule/mod.rs` 只定义 `SchedulePlugin`，并调用各注册函数。
- `schedule/mod.rs` 不直接写大段 `add_systems`。
- `schedule/sets.rs` 定义 system set、顺序标签和 set 配置。
- `schedule/update.rs` 注册 `Update` 阶段系统。
- `schedule/enter.rs` 注册 `OnEnter(...)` 阶段系统。
- `schedule/exit.rs` 注册 `OnExit(...)` 阶段系统。
- `schedule` 只决定 system 注册到哪个 Bevy schedule、哪个 state、哪个 set。
- `schedule` 不写具体 ECS 规则函数，不消费 request，不生成 prefab，不写 cleanup 细节。

## Lifecycle 目录规则

- `lifecycle` 表达 session、level、round 等 gameplay 生命周期概念。
- `lifecycle` 不直接定义 ECS 数据。
- 具体 Entity 清理仍然通过 `prefab` 或 `ecs` 暴露的窄 facade 完成。
- 状态进入/退出的注册可以在 `schedule`、`spawning`、`cleanup` 或 lifecycle plugin 中完成，但必须保持语义清楚。

## 边界规则

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- `Message` 只允许用于 `api` 边界类型；不要把 gameplay 内部状态伪装成 message。
- 不写底层 ECS 规则函数；需要调度底层规则时使用 `crates/prefab` 暴露的窄 facade。
- 不封装物理后端；这些放到 `crates/physics`。
- 不读取外部来源；外部 AI、脚本、回放和未来网络放到 `external_runtime`，并通过 manager 进入 gameplay。
- 不读取本机外设；键盘、鼠标和手柄放到 `peripherals`，再转换成语义请求。
- 不直接读取 Bevy 底层 interaction 状态；UI 和世界对象 hover/click 等交互由 `interaction` 转换成语义 message，gameplay 只消费这些 message 并执行业务。
- 不写渲染、动画、UI、相机；这些放到渲染层。
- 不直接散装实体组件；生成对象时优先调用 `crates/prefab`。
- 外部来源不要直接调用 gameplay 内部执行函数；应该通过 `api` 提交请求，由 gameplay system 统一消费。

## 依赖规则

- `gameplay` 可以依赖 `prefab`，用于 gameplay setup 中使用封装好的对象模板、spawn API 和窄 facade。
- `gameplay` 可以依赖 `intent`，用于注册和调度 Entity 意图相关能力。
- `gameplay` 可以依赖 `interaction`，用于消费 UI 和世界对象交互 message。
- `gameplay` 可以依赖 `render_2d` 的 camera 基础配置，用于编排 UI camera 和 UI prefab 的生成。
- `gameplay` 必须依赖 `error`。
- `gameplay` 不依赖 `external_runtime`；external runtime 持有 gameplay manager。
- `gameplay` 不依赖 `audio`；音频基础插件和 ECS 音频槽位桥接通过 `prefab` 组合。
- `gameplay` 不依赖 `ecs`。
- `gameplay` 不依赖 `render_3d`；除 UI camera 生成编排外，不直接依赖 `render_2d` 的具体表现内容。
- `gameplay` 不直接依赖 `physics`；对象组合通过 `prefab` 完成，并由 `gameplay` 注册 `PrefabPlugin`。

## 验证要求

修改 `crates/gameplay` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
