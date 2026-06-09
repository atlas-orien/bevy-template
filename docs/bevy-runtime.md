# Bevy Runtime 心智模型

这份文档解释 Bevy 真正负责“运行游戏”的部分。

这里的 runtime 不是本项目的 `crates/gameplay`。本项目的 `gameplay` 只是游戏玩法流程层，它把状态、spawn、system 调度规则注册给 Bevy。真正执行这些规则的是 Bevy 自己的 `App`、`World`、`Schedule` 和 runner。

## 一句话

```text
Bevy runtime = runner 驱动 App，App 每 tick 运行 Schedule，Schedule 里的 System 读写 World。
```

更展开一点：

```text
Runner
└── App::update()
    └── Main Schedule
        ├── First
        ├── PreUpdate
        ├── StateTransition
        ├── RunFixedMainLoop
        ├── Update
        ├── SpawnScene
        ├── PostUpdate
        └── Last
            └── Systems read/write World
```

## App

`App` 是 Bevy 应用的外壳。

它保存：

```text
World
Schedules
Plugins
Runner
SubApps
```

当我们写：

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GameplayPlugin)
    .run();
```

真正发生的是：

```text
1. 创建 App。
2. 插入插件。
3. 插件向 App 注册 resources、states、systems、schedules。
4. App::run() 交给 runner。
5. runner 反复调用 App::update()。
```

所以 `GameplayPlugin` 不是 runtime 本身。它只是往 Bevy `App` 里注册游戏玩法规则。

## Runner

runner 是驱动 `App` 生命周期的函数。

在无窗口或最小插件里，Bevy 可以使用 `ScheduleRunnerPlugin`，它的模型很像：

```rust
loop {
    app.update();
    if app.should_exit() {
        break;
    }
}
```

在默认桌面游戏里，通常是 `WinitPlugin` 接管系统窗口事件循环，然后在合适的时候驱动 Bevy app 更新。

所以：

```text
runner = 谁来反复调用 App::update()
```

不是我们的 `gameplay` crate。

## App::update

`App::update()` 会运行 App 的默认 schedule。

默认主 schedule 叫：

```text
Main
```

`Main` 不是一个单独 system，而是一组 schedule 的执行顺序。

Bevy 0.18 的主流程大致是：

```text
第一次运行时:
StateTransition
PreStartup
Startup
PostStartup

之后每 tick:
First
PreUpdate
StateTransition
RunFixedMainLoop
Update
SpawnScene
PostUpdate
Last
```

这就是 Bevy 真正的“主循环结构”。

## Schedule

`Schedule` 是 system 的运行容器。

我们常写：

```rust
app.add_systems(Update, movement_system);
```

这表示：

```text
把 movement_system 放进 Update schedule。
```

然后每次 `Main` 运行到 `Update` 阶段时，这个 system 就有机会执行。

如果写：

```rust
app.add_systems(
    Update,
    movement_system.run_if(in_state(AppState::Playing)),
);
```

这表示：

```text
movement_system 仍然在 Update 阶段。
但只有当前状态是 Playing 时才运行。
```

## World

`World` 是 Bevy 保存实例数据的地方。

它保存：

```text
Entities
Components
Resources
Messages / Events
Schedules
```

System 不是直接操作普通对象引用，而是通过参数从 `World` 里取数据：

```rust
fn movement_system(
    time: Res<Time>,
    mut movers: Query<(&PlayerSpeed, &mut Transform)>,
) {
    // ...
}
```

这里的 `Res` 和 `Query` 都是 Bevy 在运行 system 时，从 `World` 里取出来的。

## Plugin

`Plugin` 是注册规则，不是 runtime。

插件通常做这些事情：

```text
注册 system
注册 resource
注册 state
注册 message/event
注册其它 plugin
```

例如本项目的 `GameplayPlugin`：

```text
注册 PrefabPlugin
注册 StatePlugin
注册 SchedulePlugin
注册 SpawningPlugin
注册 InputPlugin
注册 IntentPlugin
```

它不自己跑循环，也不自己保存对象实例。

可以理解成：

```text
Plugin = 把一组规则安装到 Bevy App 里。
```

## State

Bevy 的 state 是基于 `World` 里的资源和 `StateTransition` schedule 实现的。

当我们写：

```rust
app.init_state::<AppState>();
```

Bevy 会设置：

```text
State<AppState>
NextState<AppState>
StateTransition
OnEnter(AppState::...)
OnExit(AppState::...)
OnTransition(...)
```

当某个 system 写入：

```rust
next_state.set(AppState::Playing);
```

状态不会变成一个独立 actor，也不会直接调用某个对象方法。它会在 Bevy 的 state transition 阶段被处理。

所以：

```rust
app.add_systems(OnEnter(AppState::Playing), spawn_gameplay_plan_system);
```

表示：

```text
当 StateTransition 发现进入 Playing 时，运行这个 OnEnter schedule。
```

而：

```rust
app.add_systems(OnExit(AppState::Playing), despawn_gameplay_prefabs_system);
```

表示：

```text
当 StateTransition 发现离开 Playing 时，运行这个 OnExit schedule。
```

## FixedUpdate 和 Update

`Update` 通常是每一帧运行一次。

`FixedUpdate` 是固定时间步长运行，默认适合：

```text
物理
AI
网络同步
稳定的游戏规则
```

Bevy 的主 schedule 中有一个：

```text
RunFixedMainLoop
```

它会根据积累的时间决定本 tick 里要运行 `FixedMain` 零次、一次或多次。

因此：

```text
Update = 跟随帧率
FixedUpdate = 跟随固定逻辑频率
```

本项目当前移动系统放在 `Update` 只是模板早期选择。以后如果移动、物理、网络同步需要稳定步长，可以移动到 `FixedUpdate` 或更明确的 schedule。

## Rendering

Bevy 渲染不是简单在 `Update` 里画一切。

Bevy 的渲染通常在单独的 render sub-app 中运行。主 app 负责游戏世界数据，render app 负责渲染阶段。两者之间会有数据抽取和同步。

对当前架构最重要的理解是：

```text
游戏逻辑 system 修改 World。
渲染系统从 World 或 render world 获取表现数据。
渲染不是 gameplay crate 自己直接执行。
```

所以本项目才把职责分成：

```text
gameplay  -> 状态流、spawn、调度
prefab    -> 组合 ecs/physics/render 数据
render_2d -> 表现层
```

## 本项目的 gameplay 层是什么

`crates/gameplay` 不是 Bevy runtime。

它是：

```text
注册给 Bevy runtime 执行的游戏玩法流程层。
```

它负责描述：

```text
有哪些游戏状态？
进入 Playing 时生成什么？
Playing 中哪些 system 运行？
离开 Playing 时清理什么？
```

但真正执行的是：

```text
Bevy runner -> App::update -> Main Schedule -> Systems -> World
```

因此本项目里：

```text
app       = 创建 Bevy App，注册 GameplayPlugin
gameplay  = 定义游戏玩法流程，注册 states/systems/spawn plan
prefab    = 定义对象如何生成
ecs       = 定义组件和底层系统
Bevy      = 真正运行 App/Schedule/World
```

## 和 Actor 模型的区别

Bevy runtime 不是 Erlang actor 模型。

Actor 模型通常是：

```text
Actor 有 mailbox。
Actor 接收消息。
Actor 修改自己的状态。
Actor 给其它 Actor 发消息。
```

Bevy 更像：

```text
World 保存所有数据。
Schedule 决定系统何时运行。
System 批量查询和修改数据。
Messages/Events 只是系统间通信方式之一。
```

所以服务器发来的 spawn 消息，不应该理解成：

```text
input 调用 gameplay actor 的 spawn 方法。
```

更适合理解成：

```text
input/network system 写入请求或消息。
gameplay/spawning system 在合适 schedule 中消费请求。
prefab.spawn 把组件实例插入 World。
```

## 和 MCU 主循环的关系

Bevy runtime 更接近 MCU 或游戏主循环，但它不是一个手写的单函数 loop。

手写主循环可能是：

```text
loop {
    read_input();
    update_game();
    update_physics();
    render();
}
```

Bevy 把它拆成：

```text
Runner 负责循环。
Main Schedule 负责阶段顺序。
Systems 负责具体逻辑。
World 负责保存数据。
Plugins 负责安装这些规则。
```

所以学习 Bevy runtime 时，不要先找一个传统 `main_loop()` 函数。

要看：

```text
App::run()
Runner
App::update()
Main Schedule
System
World
```

## 最重要的边界

```text
Bevy runtime:
负责执行 App/Schedule/World。

本项目 gameplay:
负责声明游戏玩法流程和调度规则。

本项目 prefab:
负责声明对象如何组合并 spawn。
```

这条边界非常重要。只要它清楚，后面设计 spawn、input、network、state、cleanup 时就不会把职责混在一起。
