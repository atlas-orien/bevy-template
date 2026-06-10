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

这就是 Bevy 真正的“主调度结构”。

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

## 常见注册路径

Bevy 的注册入口很多。可以先把它们理解成：

```text
把什么东西，注册到 App 的哪个位置？
```

下面列出项目中最常用、最应该先掌握的注册路径。

### Plugin

```rust
app.add_plugins(GameplayPlugin);
```

含义：

```text
把一组注册规则安装进 App。
```

适合：

```text
一组 systems
一组 resources
一组 states
一组 messages/events
其它 plugins
```

`Plugin::build` 只在注册阶段执行，不是每帧执行。

### System

```rust
app.add_systems(Update, my_system);
```

含义：

```text
把 my_system 注册到 Update schedule。
```

常见 schedule：

```text
PreStartup  -> Startup 之前运行一次
Startup     -> app 启动时运行一次
PostStartup -> Startup 之后运行一次

First       -> 每 tick 最早阶段
PreUpdate   -> Update 前的准备阶段，常用于输入、消息整理
Update      -> 每帧主要逻辑
PostUpdate  -> Update 后的同步或响应阶段
Last        -> 每 tick 最后阶段

FixedUpdate -> 固定时间步长逻辑，常用于物理、AI、网络同步
```

当前项目例子：

```rust
app.add_systems(
    Update,
    (
        forward_manager_requests_system,
        consume_gameplay_requests_system,
        sync_gameplay_entities_system,
    ),
);
```

含义：

```text
在 Update 阶段把 external_runtime 发来的请求转交给 gameplay，并同步 manager 可见的 entity registry。
```

### Startup

```rust
app.add_systems(Startup, setup_system);
```

含义：

```text
App 启动时运行一次。
```

适合：

```text
全局相机
全局 UI 根节点
一次性 debug setup
```

注意：如果逻辑属于某个游戏状态进入，例如进入 `Playing` 才生成玩家，不应该放 `Startup`，而应该放 `OnEnter(AppState::Playing)`。

### State

```rust
app.init_state::<AppState>();
```

含义：

```text
初始化一个 Bevy State 类型。
```

它会让 Bevy 可以使用：

```text
State<AppState>
NextState<AppState>
OnEnter(AppState::...)
OnExit(AppState::...)
OnTransition(...)
in_state(AppState::...)
```

当前项目例子：

```rust
app.init_state::<AppState>();
```

### OnEnter

```rust
app.add_systems(OnEnter(AppState::Playing), spawn_gameplay_plan_system);
```

含义：

```text
当状态进入 Playing 时运行一次。
```

适合：

```text
进入关卡时生成对象
进入菜单时生成菜单 UI
进入暂停时打开暂停界面
```

当前项目中，`gameplay/src/spawning` 使用这个入口执行默认 gameplay spawn plan；模板默认 plan 为空。

### OnExit

```rust
app.add_systems(OnExit(AppState::Playing), despawn_gameplay_prefabs_system);
```

含义：

```text
当状态离开 Playing 时运行一次。
```

适合：

```text
清理本局游戏实体
关闭状态专属 UI
释放状态专属资源
```

当前项目中，`gameplay/src/schedule` 使用这个入口清理 gameplay session 生成的实体。

### OnTransition

```rust
app.add_systems(
    OnTransition {
        exited: AppState::MainMenu,
        entered: AppState::Playing,
    },
    transition_system,
);
```

含义：

```text
只在某个明确状态切换发生时运行。
```

适合：

```text
只关心 A -> B 的特殊过渡逻辑
```

多数情况下先用 `OnEnter` 和 `OnExit` 就够了。

### Run Condition

```rust
my_system.run_if(in_state(AppState::Playing))
```

含义：

```text
system 仍然属于某个 schedule，但只有条件满足时才运行。
```

常见用途：

```text
Playing 时运行 gameplay 逻辑
Paused 时停止移动系统
MainMenu 时只运行菜单输入
```

### Resource

```rust
app.init_resource::<MyResource>();
```

或：

```rust
app.insert_resource(MyResource { ... });
```

含义：

```text
把一份全局数据放进 World。
```

适合：

```text
配置
计时器
当前关卡
全局 session 状态
唯一实体引用
```

system 里通过：

```rust
Res<MyResource>
ResMut<MyResource>
```

读取或修改。

### Message / Event

Bevy 0.18 中底层使用 message 体系；很多概念上仍可以理解成“系统之间传递事件”。

常见注册形式是：

```rust
app.add_message::<MyMessage>();
```

含义：

```text
注册一种系统间消息类型。
```

适合：

```text
输入来源产生请求
网络消息进入游戏逻辑
某个系统通知另一个系统发生了什么
```

对于本项目未来的网络 spawn，需要由 v2 单独的 network 层接入：

```text
network bridge 通过 gameplay API 提交请求
gameplay 内部 system 消费 message
prefab.spawn 插入 World
```

### Observer

Bevy 也支持 observer 风格的响应系统，用于响应特定触发。

它适合某些事件驱动或实体相关的响应逻辑。但在当前项目早期，优先使用明确的 schedule、state 和 message，避免过早引入过多运行模型。

### SubApp

Bevy 可以有多个 sub-app。渲染就是典型例子：主 app 负责游戏世界，render app 负责渲染阶段。

当前项目普通 gameplay 逻辑不需要直接操作 sub-app。

可以先记住：

```text
大多数游戏逻辑注册到 main App。
渲染等引擎模块可能使用 SubApp。
```

## 注册路径和本项目目录的关系

本项目目录不按 Bevy schedule 名字分类，而按游戏语义分类。

例如：

```text
gameplay/src/spawning
```

表示“生成流程”，不是“OnEnter 目录”。它当前注册到：

```text
OnEnter(AppState::Playing)
```

以后它也可能注册定时 spawn、网络 spawn 或 message-driven spawn。

再比如：

```text
gameplay/src/schedule
```

表示“玩法系统运行规则”，里面可以把系统注册到：

```text
Update
FixedUpdate
OnExit
```

因此新增 system 时要同时回答两个问题：

```text
它属于哪个游戏语义模块？
它注册到 Bevy 的哪个路径？
```

不要只按 `Update/OnEnter/OnExit` 建目录。Bevy schedule 是运行路径，项目目录表达业务语义。

## Gameplay API

本项目有一层 gameplay API。

它的作用是：

```text
外部来源或上层逻辑请求 gameplay 改变游戏流程或世界。
```

它不是 Bevy `Commands`，也不是 Bevy `World`。

它是本项目定义的外部边界：

```text
外部来源只调用 gameplay API。
gameplay API 把请求送进 Bevy。
gameplay 内部 system 决定何时执行。
```

当前实现可以使用 Bevy `Message` 作为内部通道，但 `Message` 只是连接方式，不是这个概念本身。

典型路径：

```text
外部来源
-> 调用 gameplay API
-> 提交 GameplayRequest
-> gameplay 内部 system 在明确 schedule 中消费请求
-> gameplay 调用 prefab、state 或其它内部能力
-> Bevy World 发生变化
```

例如运行中由服务器消息触发生成对象时，理想流程是：

```text
external layer
-> gameplay::api::GameplayRequest::spawn_prefab(prefab)
-> gameplay::api::submit_gameplay_request(...)
-> gameplay/spawning system
-> prefab.spawn(commands)
-> Main World Entity
```

这里的关键边界是：

```text
外部来源不直接拿 Commands。
外部来源不直接调用 Prefab::spawn。
外部来源只调用 gameplay API。
gameplay system 决定何时消费并执行。
```

API 适合表达高层 gameplay 请求，例如：

```text
spawn / despawn
change state
load level
teleport
grant item
start dialog
```

已有 Entity 的连续意图，例如移动、瞄准、普通攻击输入，不一定属于 API 请求；这类更适合由 intent 表达。

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
注册 GameplayApiPlugin
注册 StatePlugin
注册 SchedulePlugin
注册 SpawningPlugin
注册 CleanupPlugin
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

因此，spawn 一个带有渲染表现的 prefab 时，通常仍然是生成到 Main World：

```text
prefab.spawn(commands)
-> Main World Entity
   + Transform
   + Visibility
   + Sprite / render marker / render-facing bundle
```

然后 Bevy/render 层再从 Main World 抽取需要的表现数据到 Render SubApp。

所以本项目里：

```text
prefab 可以组合 Main World 上的表现组件。
prefab 不直接操作 RenderApp、Render World、render graph、pipeline 或 GPU resource。
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

## 最重要的边界

```text
Bevy runtime:
负责执行 App/Schedule/World。

本项目 gameplay:
负责声明游戏玩法流程和调度规则。

本项目 prefab:
负责声明对象如何组合并 spawn。
```

这条边界非常重要。只要它清楚，后面设计 spawn、external_runtime、network、state、cleanup 时就不会把职责混在一起。
