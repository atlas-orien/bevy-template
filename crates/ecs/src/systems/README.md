# systems

`systems` 是 ECS 里的系统函数层。

如果说 `crates/ecs/src/components` 定义“世界里有什么数据”，那么 `crates/ecs/src/systems` 就定义“这些数据如何被规则改变”。

## 什么是 System

在 Bevy ECS 里，`System` 通常就是一个普通 Rust 函数。

它通过参数声明自己需要读取或修改哪些数据：

```rust
fn movement_system(
    time: Res<Time>,
    mut query: Query<(&MovementIntent, &Speed, &mut Transform)>,
) {
    // 根据意图和速度修改 Transform
}
```

系统不直接拥有实体，也不需要写成某个对象的方法。它只关心查询条件：

```text
只要某个 Entity 同时拥有 MovementIntent、Speed、Transform，
movement_system 就可以处理它。
```

系统不绑定到某个实体类型；它批量处理所有满足查询条件的实体。

## 职责

- 定义真正修改 ECS 世界的系统函数。
- 读取 `crates/ecs/src/components`、`crates/ecs/src/resources`、`crates/ecs/src/events` 里的数据。
- 根据游戏规则修改组件、资源或发送事件。
- 保持系统函数小而明确，方便组合、测试和 AI 生成。

适合放这里：

- `movement_system`
- `damage_system`
- `healing_system`
- `death_system`
- `cooldown_system`
- `pickup_item_system`
- `use_item_system`
- `spawn_system`
- `despawn_system`

## 当前分类

- `movement`: 移动、速度、位移、朝向结算。
- `combat`: 伤害、治疗、攻击、防御等战斗规则。
- `lifecycle`: 出生、死亡、销毁、状态切换。
- `inventory`: 拾取、使用、装备、背包变化。
- `interaction`: 交互、触发器、区域进入离开。
- `spawning`: 根据数据生成或销毁实体。

## 和 components 的区别

`crates/ecs/src/components` 只定义数据：

```rust
struct Health {
    current: f32,
}
```

`crates/ecs/src/systems` 定义规则：

```rust
fn damage_system(...) {
    // 读取 DamageEvent，修改 Health
}
```

如果代码只是描述“实体拥有什么数据”，放到 `crates/ecs/src/components`。

如果代码开始“读取数据并改变世界”，放到 `crates/ecs/src/systems`。

## 和 intent 的区别

`intent` 表达某个 `Entity` 想做什么。

外部控制来源不属于 `crates/ecs/src/systems`，也不属于 `intent` 本身。它们应该放到 `external_runtime`，再通过 manager 转换成 gameplay 请求或意图数据。

例如：

- 某个可控制角色想移动到一个方向或位置，写入 `MovementIntent`
- 某个可控制角色想攻击，写入 `AttackIntent`
- 某个可控制角色想交互，写入 `InteractIntent`

`crates/ecs/src/systems` 不关心意图来自哪里，只负责根据意图和规则改变世界。

例如：

```text
intent 写入 MovementIntent
crates/ecs/src/systems 根据 MovementIntent + Speed + Time 修改 Transform
```

也就是说：

```text
intent = 哪个 Entity 想做什么
ecs/systems = 根据规则怎么做
gameplay = 什么时候运行这些规则
```

## 和 render 的区别

`crates/ecs/src/systems` 负责游戏规则，不负责表现。

不适合放这里：

- sprite 生成
- 动画播放
- 相机跟随
- UI 布局
- 模型、材质、光照

这些应该放到 `render_2d` 或 `render_3d`。

## 规则

- 不定义 `Component`、`Bundle`、`Resource`、`Event`，这些放到 `crates/ecs/src/components`、`crates/ecs/src/resources`、`crates/ecs/src/events`。
- 不读取键盘、鼠标、手柄、外设、AI、脚本、网络等外部来源；外部来源先由对应边界层转换成 intent 或 gameplay API 请求。
- 不写渲染、动画、UI、相机，这些放到渲染层。
- 不把多个无关规则塞进一个大系统函数。
- 系统函数命名使用 `_system` 后缀，例如 `movement_system`。
