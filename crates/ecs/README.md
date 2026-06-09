# ecs

`ecs` 是 Bevy 游戏核心层。

这个 crate 同时放 ECS 的数据定义和系统函数：

- `crates/ecs/src/components`: 挂在实体上的数据。
- `crates/ecs/src/resources`: 全局 ECS 数据。
- `crates/ecs/src/events`: 系统之间传递的事件数据。
- `crates/ecs/src/systems`: 读取和修改 ECS 数据的系统函数。

## 什么是 ECS

ECS 是 Entity Component System 的缩写，是 Bevy 使用的核心架构。

传统面向对象写法里，我们可能会设计一个很大的 `Player` 类型，把血量、移动、输入、动画、背包、攻击逻辑都放进去。ECS 的思路不同：游戏对象不是一个“大类”，而是一个实体 ID 加上一组小的数据组件，再由系统函数去处理这些数据。

### Entity

`Entity` 是实体，可以理解成游戏世界里的一个对象 ID。

它本身不保存业务数据，只负责表示“有这么一个东西存在”。

例如这些都可以是实体：

- 玩家
- 敌人
- NPC
- 武器
- 子弹
- 掉落物
- 地图触发区域
- UI 节点

一个实体到底是什么，取决于它身上挂了哪些组件。

### Component

`Component` 是挂在实体上的数据。

组件应该尽量小、清晰、可组合。它只描述数据，不负责执行行为。

例如：

- `Health`: 当前生命值
- `MaxHealth`: 最大生命值
- `Speed`: 移动速度
- `MovementIntent`: 想往哪个方向移动，或想移动到哪个位置
- `Player`: 玩家标记
- `Enemy`: 敌人标记
- `Faction`: 阵营
- `Inventory`: 背包数据

在 ECS 里，一个“玩家”不是一个很大的 `Player` 类，而是一个实体加上一组组件：

```text
Entity
+ Player
+ Health
+ MaxHealth
+ MovementIntent
+ Speed
+ Transform
```

一个“敌人”也可以复用很多相同组件：

```text
Entity
+ Enemy
+ Health
+ MaxHealth
+ MovementIntent
+ Speed
+ Transform
```

这样 `movement_system` 可以同时处理玩家和敌人，因为它只关心实体有没有 `MovementIntent`、`Speed`、`Transform`，不关心它是不是玩家。

### System

`System` 是读取和修改组件的函数。

系统负责“行为”，例如：

- 外部来源转换出意图后，写入某个 Entity 的 `MovementIntent`
- 根据 `MovementIntent` 和 `Speed` 修改 `Transform`
- 根据 `DamageEvent` 修改 `Health`
- 根据 `Health` 判断实体是否死亡
- 根据 `Health` 更新 UI 血条

系统函数放在 `crates/ecs/src/systems` 目录中。输入控制、渲染表现和 app 组装仍然放在其他 crate。

### Bundle

`Bundle` 是一组组件的初始化组合。

它不是新的运行时能力，只是方便生成实体时一次性插入多种组件。

例如 `PlayerBundle` 可以组合：

```text
Player
Health
MaxHealth
Speed
MovementIntent
Facing
Transform
Visibility
```

所以 `crates/ecs/src/components/characters/player.rs` 可以定义 `Player` 和 `PlayerBundle`。如果要写 `spawn_player_system`，应该放到 `crates/ecs/src/systems/spawning` 或更高层的 gameplay 流程 crate。

### Resource

`Resource` 是 Bevy World 里的全局数据，通常只有一份。

它不挂在某个实体上，而是描述整个游戏当前共享的状态或配置。

例如：

- `GameConfig`
- `CurrentLevel`
- `GameSession`
- `Difficulty`
- `InputConfig`

注意：这里的 `Resource` 不是根目录 `assets/` 里的图片、音频、字体文件。`Resource` 是 Rust 代码里的 ECS 数据类型。

### Event

`Event` 是系统之间传递的消息数据。

事件只描述“发生了什么”，不直接处理后果。

例如：

- `DamageEvent`: 某个实体受到伤害
- `HealEvent`: 某个实体被治疗
- `DiedEvent`: 某个实体死亡
- `ItemPickedEvent`: 某个物品被拾取

真正处理事件的逻辑应该放在 `crates/ecs/src/systems` 中，比如 `damage_system` 读取 `DamageEvent`，然后修改 `Health`。

## 为什么放在同一个 ecs crate

这个模板希望人和 AI 都能快速判断代码应该放在哪里。

Bevy 的核心是 ECS。把 ECS 数据和 ECS 系统函数放在同一个 crate 里，可以让初学者先理解完整的 ECS 模型，再去理解输入、渲染和 app 组装。

这里采用一个简单规则：

```text
crates/ecs/src/components = 世界里有什么实体数据
crates/ecs/src/resources = 世界里有什么全局数据
crates/ecs/src/events = 世界里发生了什么消息
crates/ecs/src/systems = ECS 数据如何根据规则改变
intent = 实体想做什么
render_2d/render_3d = 数据如何显示出来
app = 如何组装并启动游戏
```

`ecs` 内部仍然要保持目录边界：组件数据写进 `crates/ecs/src/components`，全局资源写进 `crates/ecs/src/resources`，事件写进 `crates/ecs/src/events`，行为写进 `crates/ecs/src/systems`。不要把系统函数塞进 `components`，也不要把组件定义塞进 `systems`。

判断一个类型是否适合放到 `components`，可以问：

```text
它是不是只描述数据？
它能不能被多个系统读取？
它有没有执行行为？
```

如果答案是“只描述数据，没有执行行为”，通常就适合放到这里。

## 职责

- 定义 Bevy `Component`、`Bundle`、`Resource`、`Event`、标记组件。
- 定义真正读取和修改 ECS 数据的系统函数。
- 按游戏数据和系统职责分类。
- 提供其他 crate 共享使用的 ECS 核心能力。

## 当前结构

- `crates/ecs/src/components/base`: 基础组件，例如血量、属性、移动、身份、阵营。
- `crates/ecs/src/components/characters`: 角色类实体数据，例如 `Player`、`Enemy`、`Npc` 以及对应 Bundle。
- `crates/ecs/src/components/items`: 物品类实体数据，例如武器、防具、消耗品、掉落物。
- `crates/ecs/src/components/world`: 世界、地图、关卡、区域、出生点等数据。
- `crates/ecs/src/components/ui`: UI 相关 ECS 数据，例如 HUD、血条、菜单、按钮动作。
- `crates/ecs/src/events`: ECS 事件数据，例如伤害、治疗、死亡、拾取。
- `crates/ecs/src/resources`: Bevy ECS 全局 `Resource` 数据，例如配置、当前关卡、运行会话。
- `crates/ecs/src/systems/movement`: 移动、速度、位移、朝向结算。
- `crates/ecs/src/systems/combat`: 伤害、治疗、攻击、防御等战斗规则。
- `crates/ecs/src/systems/lifecycle`: 出生、死亡、销毁、重生等生命周期规则。
- `crates/ecs/src/systems/inventory`: 拾取、使用、装备、背包变化。
- `crates/ecs/src/systems/interaction`: 交互、触发器、区域进入离开。
- `crates/ecs/src/systems/spawning`: 根据 ECS 数据生成或销毁实体。

## 不应该放这里

- 不读取键盘、手柄、鼠标。
- 不写 AI 控制。
- 不加载图片、模型、音频。
- 不写渲染动画。

判断规则：

- 如果代码描述“一个东西拥有什么数据”，放到 `crates/ecs/src/components`、`crates/ecs/src/resources` 或 `crates/ecs/src/events`。
- 如果代码根据规则读取和修改 ECS 数据，放到 `crates/ecs/src/systems`。
- 如果代码读取外部来源，放到 `external_runtime`，并通过 manager 转换成 gameplay 请求或 intent。
- 如果代码处理显示、动画、相机、UI 布局，放到 `render_2d` 或 `render_3d`。

## 和 assets 的区别

`assets/` 是磁盘文件目录，放图片、音频、字体、地图文件等外部资源。

`ecs/src/resources/` 是 Rust 数据目录，放 `#[derive(Resource)]` 的 ECS 全局数据类型。
