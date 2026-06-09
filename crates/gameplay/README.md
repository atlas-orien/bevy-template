# gameplay

`gameplay` 是游戏玩法流程层。

它负责把 gameplay state、API 请求、session 进入/退出、spawn plan、cleanup 和 system 调度注册给 Bevy。

它不是 Bevy 底层 runtime。Bevy 的 `App`、`World`、`Schedule` 和 runner 负责真正执行系统。

## 目录结构

```text
src/
├── api
├── cleanup
├── lifecycle
├── schedule
├── spawning
└── state
```

## api

外部来源进入 gameplay 的统一 API 边界。

`api` 只表达“希望 gameplay 做什么”，不直接修改 `World`。

当前文件：

- `channel.rs`: 定义 gameplay request sender 和 Bevy App 内部的 request inbox。
- `mod.rs`: 注册 `GameplayApiPlugin`，并注册 `RuntimeRequest` message。
- runtime/world 消息类型定义在 `gameplay::api::runtime_channel`，底层 channel 机制来自 `helper`。
- `submit.rs`: 提供提交 gameplay request 的窄函数。
- `systems.rs`: 消费 `RuntimeRequest`，并调用 gameplay 内部能力。

API 不暴露 Bevy `Entity` 给外部来源。外部请求必须使用 gameplay-facing id，gameplay 内部负责映射到 Bevy `Entity`。

外部 runtime 不直接拿 `MessageWriter`。`main` 从 `gameplay::api::duplex()` 获得 runtime/world 两端 endpoint；external runtime 持有 runtime endpoint，Bevy App 持有 world endpoint，并在 `Update` 中转发为 `RuntimeRequest` message。

当前最小请求：

- `SpawnPrefab`: 运行中生成 prefab。
- `DespawnEntity`: 按 gameplay-facing id 销毁实体。
- `ClearSession`: 清理当前 gameplay session 生成的实体。
- `ChangeState`: 请求切换 gameplay state。
- `SetMovementIntent`: 按 gameplay-facing id 设置移动意图。

新增 API 请求时：

- 请求类型写到 `gameplay::api::runtime_channel`。
- 请求提交函数如果需要封装，写到 `api/submit.rs`。
- 外部 runtime 入口如果需要扩展，写到 `crates/external_runtime/src/manager`。
- 请求执行逻辑写到 `api/systems.rs`。
- 不要让外部来源直接调用 gameplay 内部 system。

## state

游戏状态定义和状态切换入口。

当前状态：

- `Loading`
- `MainMenu`
- `Playing`
- `Paused`
- `GameOver`

当前文件：

- `mod.rs`: 定义 `AppState` 和 `StatePlugin`。

状态相关规则：

- 新增全局 gameplay state 时，先写到 `AppState`。
- 状态进入时的一次性逻辑使用 `OnEnter(AppState::...)`。
- 状态退出时的清理逻辑使用 `OnExit(AppState::...)`。
- 每帧运行逻辑使用 schedule + `in_state(...)`。

## schedule

系统集合、运行条件和调度顺序。

当前文件：

- `mod.rs`: 定义 `SchedulePlugin`，注册 gameplay 每帧系统和退出清理系统。

当前注册内容：

- `Update`: 消费 gameplay API 请求。
- `Update + Playing`: 运行 prefab 暴露的 movement system 示例。
- `OnExit(Playing)`: 清理 gameplay session entity。

新增 system 调度时：

- 先判断 system 属于哪个语义目录。
- 再在 `schedule` 中决定它注册到哪个 Bevy schedule。
- 不要按 `Update/OnEnter/OnExit` 新建目录。
- 不要在这里写 ECS 规则函数，只注册已有 system。

## spawning

gameplay session 进入时的生成流程。

当前文件：

- `mod.rs`: 组装 `SpawningPlugin`。
- `plan.rs`: 定义 `GameplaySpawnPlan`。
- `gameplay::api::SpawnItem`: object-safe spawn item，用于 runtime request 保存任意 prefab。
- `defaults.rs`: 定义模板默认 spawn plan。
- `systems.rs`: 执行默认 spawn plan。

当前行为：

- `OnEnter(Playing)` 时执行 `default_gameplay_spawn_plan()`。

新增默认生成内容时：

- 优先改 `spawning/defaults.rs`。
- 新增具体 prefab 时，不维护中心 enum 或 match 列表。
- 只要 prefab 实现 `prefab::Prefab`，就可以进入 `GameplaySpawnPlan`。

运行中 spawn 不放在 `spawning` 消费，走 `RuntimeRequest::SpawnPrefab`。

## cleanup

清理策略入口。

当前文件：

- `mod.rs`: 定义 `CleanupPlugin`。

当前第一版没有额外系统。退出 `Playing` 时的 session entity 清理由 `schedule` 注册。

新增清理策略时：

- 如果是状态退出清理，优先使用 `OnExit(...)`。
- 如果是外部请求触发清理，优先走 `RuntimeRequest::ClearSession`。
- 不要在 cleanup 中散装组件查询逻辑；需要底层能力时通过 `prefab` 暴露的窄 facade。

## lifecycle

session、level、round 等玩法生命周期的标准落点。

当前文件：

- `mod.rs`: 定义 `LifecyclePlugin`。

当前第一版只保留插件入口。

以后新增这些内容时放这里：

- gameplay session 生命周期。
- level 生命周期。
- round 生命周期。
- 进入下一局、重开本局、结束本局等流程。

不要把关卡、回合、游戏局生命周期塞进 `spawning` 或 `schedule`。

## 不应该放这里

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- 不直接依赖或调用 `ecs`。
- 不封装 Avian、Rapier 或其它物理后端。
- 不直接读取键盘、手柄、鼠标、外设、AI、脚本或网络来源。
- 不加载精灵、模型、音频。
- 不播放动画。
- 不写 UI、相机、材质、灯光。
- 不在生成系统里散装大量组件，应该调用 `prefab`。
