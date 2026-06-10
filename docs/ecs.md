# ECS 心智模型

这份文档不是 Bevy API 手册，而是帮助从普通 Rust 对象模型切换到 Bevy ECS 模型。

如果想先理解 Bevy 真正如何运行 `App`、`Schedule` 和 `System`，先读 [bevy-runtime.md](bevy-runtime.md)。

很多初学者真正卡住的地方不是语法，而是这些问题：

- `Entity` 到底是不是对象实例？
- `Component` 是一个共享实例，还是每个对象都有一份？
- `spawn` 之后数据放在哪里？
- 为什么不是 `player.move()`，而是 `Query` 和 `System`？
- `Prefab` 和真正生成出来的对象是什么关系？

## Bevy 到底是什么

可以先大胆地把 Bevy 理解成：

```text
一个以 ECS 和调度为核心的游戏应用框架。
```

它不是传统意义上以 scene tree 或 root node 为中心的游戏引擎。

Bevy 的核心更接近：

```text
World + Entity + Component + Resource
+ System + Query
+ Schedule
+ App + Plugin
```

也就是：

```text
管理数据。
调度系统。
组织插件。
驱动每一帧执行。
```

渲染、窗口、输入、音频、资源加载等能力，都是围绕这套核心接进去的。

更准确地说，Bevy 不只是一个 ECS 管理器。它也提供很多引擎模块：

```text
bevy_ecs      -> ECS
bevy_app      -> App / Plugin
bevy_time     -> 时间
bevy_input    -> 输入
bevy_asset    -> 资源加载
bevy_render   -> 渲染框架，底层基于 wgpu
bevy_sprite   -> 2D sprite
bevy_pbr      -> 3D PBR
bevy_ui       -> UI
bevy_audio    -> 音频
```

但学习和设计架构时，最重要的思想核心通常不是“怎么画图”，而是：

```text
World 管理实例。
Component 表达数据。
System 表达规则。
Schedule 决定何时运行。
Plugin 只注册能力。
Prefab 实例组合生成 Entity。
```

所以在这个项目里，架构设计优先围绕 ECS 和调度理解，而不是回到传统 root/tree 的对象管理方式。

## 普通 Rust 对象模型

如果只写普通 Rust，我们可能会这样创建数据：

```rust
struct Speed(f32);

let a = Speed(100.0);
let b = Speed(180.0);
let c = Speed(250.0);
```

这里有一个 `Speed` 类型定义，但有三份 `Speed` 数据实例。

我们通过变量名访问它们：

```rust
println!("{}", a.0);
```

如果有 100 个对象拥有速度，就会有 100 份 `Speed` 数据。它们不是共享同一个 `Speed` 实例。

## Bevy ECS 对象模型

在 Bevy 里，数据不是由我们直接放在普通变量里长期管理，而是交给 `World` 管理。

```rust
commands.spawn((Speed(100.0),));
commands.spawn((Speed(180.0),));
commands.spawn((Speed(250.0),));
```

每次 `spawn` 都会创建一个新的 `Entity`，并把组件数据存进 Bevy 的 `World`。

可以把它理解成：

```text
World
├── Entity(1)
│   └── Speed(100.0)
├── Entity(2)
│   └── Speed(180.0)
└── Entity(3)
    └── Speed(250.0)
```

所以：

```text
Speed 类型定义只有一份。
每个拥有 Speed 的 Entity 都有自己的一份 Speed 数据。
Entity 不是 Speed 实例，也不是对象本体。
Entity 是 World 里一组组件数据的 ID。
```

## Entity

`Entity` 可以理解成 Bevy `World` 里的对象编号。

它本身不保存业务数据，也没有业务方法。它只是告诉 Bevy：

```text
World 里有这么一个东西。
这个东西身上挂了哪些组件，决定它是什么。
```

例如：

```text
Entity(42)
├── Player
├── PlayerSpeed(180.0)
├── MovementIntent
├── Transform
├── PhysicsRigidBody
└── Character2dRender
```

这个 Entity 被我们理解成“玩家”，不是因为 `Entity` 自己叫玩家，而是因为它拥有 `Player`、`PlayerSpeed`、`MovementIntent` 等组件。

## Component

`Component` 是挂在 Entity 上的数据。

它可以是纯数据：

```rust
#[derive(Component)]
struct Speed(f32);
```

也可以是标记：

```rust
#[derive(Component)]
struct Player;
```

如果 100 个 Entity 都有 `Speed`，那就是 100 份 `Speed` 数据：

```text
Entity(1)   -> Speed(100.0)
Entity(2)   -> Speed(180.0)
Entity(3)   -> Speed(250.0)
Entity(100) -> Speed(80.0)
```

这些数据由 Bevy `World` 保存。修改一个 Entity 的 `Speed`，不会影响另一个 Entity 的 `Speed`。

## Query

普通 Rust 里，我们通过变量名访问数据：

```rust
speed.0
```

Bevy ECS 里，数据在 `World` 里，所以系统通过 `Query` 按组件类型取数据：

```rust
fn movement_system(query: Query<&Speed>) {
    for speed in &query {
        println!("{}", speed.0);
    }
}
```

`Query<&Speed>` 的意思是：

```text
从 World 里找出所有拥有 Speed 组件的 Entity，并取出它们各自的 Speed 数据。
```

也可以同时查询多个组件：

```rust
fn movement_system(mut query: Query<(&Speed, &mut Transform)>) {
    for (speed, mut transform) in &mut query {
        transform.translation.x += speed.0;
    }
}
```

这表示：

```text
只处理同时拥有 Speed 和 Transform 的 Entity。
```

## System

`System` 是处理组件数据的函数。

在面向对象写法里，我们可能会写：

```rust
player.move_right();
```

在 ECS 里，通常不是 Entity 调用方法，而是 system 批量处理符合条件的 Entity：

```rust
fn movement_system(mut query: Query<(&Speed, &mut Transform)>) {
    for (speed, mut transform) in &mut query {
        transform.translation.x += speed.0;
    }
}
```

可以这样理解：

```text
Component = 数据和资格
System = 执行规则的人
Query = 找到符合规则的数据
Entity = 这些数据所属的 ID
```

一个 Entity 拥有某些组件，就获得了被对应 system 处理的资格。

## Commands

`Commands` 是修改 `World` 的命令接口。

常见用途：

```rust
commands.spawn((Speed(180.0), Transform::default()));
```

生成一个 Entity，并插入组件。

```rust
commands.entity(entity).insert(Speed(240.0));
```

给已有 Entity 插入或替换组件。

```rust
commands.entity(entity).despawn();
```

删除 Entity。

所以：

```text
spawn 时，把组件实例交给 World。
运行时，通过 Entity 或 Query 从 World 里访问组件实例。
```

## Prefab

在这个项目里，`prefab` 不是 Bevy 内置概念，而是项目自己的对象模板层。

`Prefab` 表达：

```text
如何把 ecs + physics + render 的组件组合起来，生成一个完整对象。
```

例如具体项目可以定义自己的玩家 prefab。prefab 不是已经进入 World 的玩家实例，而是一个待生成的 prefab 实例，保存生成对象所需的数据：

```text
PlayerPrefab = 待生成的玩家 prefab 实例，包含生成所需的数据
PlayerPrefabBundle = 要放进 World 的组件组合
Entity = 生成后返回的主对象 ID
World = 真正保存组件实例的地方
```

生成后大致是：

```text
World
└── Entity(42)
    ├── GameplayEntity
    ├── GameplaySessionEntity
    ├── Player
    ├── LocalPlayerControlled
    ├── PlayerSpeed(180.0)
    ├── MovementIntent
    ├── Facing
    ├── Transform
    ├── PhysicsRigidBody
    ├── PhysicsCollider2d
    ├── PhysicsLayer
    └── Character2dRender
```

`Prefab::spawn` 返回的 `Entity` 不是对象实例本身，而是这组组件实例在 `World` 里的 ID。

## 和传统 Root/Scene Tree 的区别

很多传统游戏工具会使用 `root`、`scene`、`node tree` 来组织对象。

例如：

```text
Root
└── Level
    ├── Player
    ├── Enemy
    └── Camera
```

这种结构的一个重要作用是：

```text
给对象一个层级位置。
让代码可以从 root 往下找到实例。
让父节点管理子节点。
让生命周期跟着树结构传播。
```

在这种模型里，要找玩家，可能会写成：

```text
root.get_node("Level/Player")
```

或者让父节点保存子节点引用。

这是一种“对象在树里”的思路。

Bevy ECS 的思路不同。它不要求先从 root 往下找到对象，而是问：

```text
World 里谁拥有我需要的这些组件？
```

例如：

```rust
Query<(&Player, &Transform)>
```

表达的是：

```text
从整个 World 里找出所有同时拥有 Player 和 Transform 的 Entity。
```

所以核心差异是：

```text
传统 Root/Scene Tree:
对象靠层级组织和路径查找。

Bevy ECS/World:
对象靠组件组合识别和 Query 查找。
```

删除传统 scene/root 之后，最容易产生的问题是：

```text
没有 root 了，我怎么找到对象？
```

在 ECS 里，答案通常不是重新造一个 root，而是使用：

```text
Component marker
Query
Resource
Entity ID
```

例如，想找到玩家，可以给玩家 Entity 挂上 `Player` marker：

```rust
#[derive(Component)]
struct Player;
```

然后通过查询找到它：

```rust
fn find_player(query: Query<Entity, With<Player>>) {
    for player in &query {
        // player 是玩家 Entity ID
    }
}
```

如果某个对象确实是全局唯一、经常需要被直接定位，也可以用 `Resource` 保存它的 Entity：

```rust
#[derive(Resource)]
struct PlayerEntity(Entity);
```

这不是回到 root tree，而是明确告诉 ECS：

```text
这个 Entity 是一个全局关心的对象。
```

因此：

```text
Root/Tree 解决的是路径和父子层级查找。
ECS 解决的是数据组合和批量查询。
```

这也是本项目不希望重新用 root 添加节点来表达世界结构的原因。对象实例由 `World` 管理，对象身份由组件组合表达，对象查找由 `Query` 或必要的 `Resource` 完成。

## 一句话总结

```text
Bevy 替我们管理实例。
Entity 是实例 ID。
Component 是挂在 Entity 上的数据实例。
World 保存所有 Entity 和 Component。
Query 从 World 里找出需要的数据。
System 对查询出来的数据执行逻辑。
Prefab 是项目里用来组合组件并生成 Entity 的待生成对象描述。
```

## Gameplay Spawn Plan

在这个项目里，`prefab` 只回答：

```text
这个对象如何生成？
```

`gameplay` 回答：

```text
什么时候生成？
这次 gameplay session 要生成哪些对象？
```

因此用户不应该在 `app` 里直接调用 `spawn`，也不应该让 `prefab` 自己决定生成时机。

当前约定是：

```text
crates/gameplay/src/spawning
```

管理 gameplay 的生成流程。

目录职责：

```text
mod.rs      -> 组装 SpawningPlugin
plan.rs     -> 定义 GameplaySpawnPlan
prefab.rs   -> 定义 object-safe spawn item 抽象
defaults.rs -> 定义模板默认 spawn plan
systems.rs  -> 定义执行 spawn plan 的 Bevy system
```

也就是：

```text
用户描述 plan。
gameplay system 执行 plan。
prefab 实例执行自己的 spawn。
Bevy World 保存生成出来的组件实例。
```

模板默认 plan 为空。以后新增玩家、敌人、道具、关卡物件时，不需要维护一个中心 enum 或 match 列表。只要新的 prefab 实现 `prefab::Prefab`，就可以放进 `GameplaySpawnPlan`。

如果从普通 Rust 迁移理解，可以先记住：

```text
普通 Rust:
struct 实例由你保存，通过变量访问。

Bevy ECS:
component 实例由 World 保存，通过 Entity 或 Query 访问。
```
