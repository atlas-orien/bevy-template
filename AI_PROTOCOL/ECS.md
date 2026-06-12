此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# ECS

这个文件是 `crates/ecs` 的 AI 规则。

`crates/ecs` 是 Bevy ECS 核心层，包含：

- `crates/ecs/src/components`: 挂在 Entity 上的组件数据。
- `crates/ecs/src/resources`: Bevy World 里的全局 ECS 数据。
- `crates/ecs/src/events`: 系统之间传递的事件数据。
- `crates/ecs/src/systems`: 读取和修改 ECS 数据的系统函数。

## 默认结构可以调整

当前目录结构是模板默认配置，不是不可修改的最终架构。

AI 可以根据具体游戏需求添加、修改或删除 `crates/ecs` 下的目录和模块，但必须遵守这些规则：

- 调整前先判断新结构是否让 ECS 职责更清楚。
- 不要为了保留模板形状而维护无用目录。
- 不要把 ECS 数据和 ECS 系统规则混到同一个文件里。
- 如果删除或改名目录，必须同步更新代码引用、Cargo 依赖和相关文档路径。
- 如果调整 `crates/ecs` 的结构，必须同步更新 `crates/ecs/README.md` 和本文件。

## 代码落点

- 新 `Component`、`Bundle`、marker：写到 `crates/ecs/src/components`。
- 新 `Resource`：写到 `crates/ecs/src/resources`。
- 新 `Event`：写到 `crates/ecs/src/events`。
- 新 ECS system 函数：写到 `crates/ecs/src/systems`。

## 骨架

```rust
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct YourValue(pub f32);

pub fn your_rule_system(
    time: Res<Time>,
    mut items: Query<(&YourValue, &mut Transform)>,
) {
    for (value, mut transform) in &mut items {
        transform.translation.x += value.0 * time.delta_secs();
    }
}
```

## 边界规则

- `crates/ecs/src/components` 只描述实体数据，不做行为。
- `crates/ecs/src/resources` 只描述全局 ECS 数据，不加载文件资源。
- `crates/ecs/src/events` 只描述发生了什么，不处理后果。
- `crates/ecs/src/systems` 负责根据 ECS 数据执行世界规则。
- 不读取键盘、鼠标、手柄、AI、脚本、网络等来源。
- 本机键盘、鼠标和手柄由 `peripherals` 转换成语义请求。
- UI 和世界对象 hover/click 等 Bevy interaction 由 `interaction` 转换成语义 message。
- AI、脚本、回放和未来网络等 Bevy App 外部来源由 `external_runtime` 转换成 manager API 请求。
- 网络是双向通信层，v2 单独设计。
- 不写 sprite、动画、相机、UI 布局、材质、光照；这些放到渲染层。

## 命名规则

- 系统函数使用 `_system` 后缀，例如 `movement_system`、`damage_system`。
- 不要恢复独立的 `components` 或 `system` crate；ECS 数据和系统统一放在 `crates/ecs`。
- 不要新增 `game_` 前缀的 ECS 模块名。
- `components/base` 按最小语义组拆文件，不按“一类型一文件”机械拆分。
- 一个 base 文件只能放同一语义组的基础组件和强绑定辅助类型。
- 推荐语义组示例：`identity.rs` 放 `DisplayName`、`PublicEntityId`；`health.rs` 放 `Health`、`MaxHealth`；`movement.rs` 放 `Speed`、`Velocity2d`、`Velocity3d`、`MovementIntent`、`MovementTarget`、`Facing`；`affiliation.rs` 放 `Faction`、`Team`；`audio.rs` 放 `AudioClip`、`AudioClips`。
- `ecs` 不按 2D / 3D 创建目录分类。2D / 3D 只是同一语义组里的数据形状变体，例如 `Velocity2d` 和 `Velocity3d` 都放在 `base/movement.rs`。
- 如果 2D / 3D 差异属于渲染、物理后端或 prefab 组合，放到 `render_2d`、`render_3d`、`physics` 或 `prefab/world_2d`、`prefab/world_3d`，不要放到 `ecs` 目录结构里。
- 不要把无关基础组件混在一个文件里，例如不要在 `movement.rs` 写血量、攻击、背包、AI 或 gameplay manager 数据。
- 如果新增 base 文件，文件名必须表达一个清楚的基础语义组。
- `resources` 按最小语义组拆文件，一个文件只放同一语义组的 Resource。
- 推荐 Resource 语义组示例：`world.rs` 放 `WorldConfig`；`session.rs` 放 `GameSession`。
- Resource 只描述 Bevy World 里的全局 ECS 数据，不加载磁盘资源文件。
- 默认 Resource 注册写在 `resources/mod.rs` 的 `ResourcesPlugin`。
- `events` 按最小语义组拆文件，一个文件只放同一语义组的 Event。
- 推荐 Event 语义组示例：`combat.rs` 放 `DamageEvent`、`HealEvent`；`lifecycle.rs` 放 `SpawnedEvent`、`DiedEvent`。
- Event 只描述“发生了什么”，不处理后果；事件处理系统写到 `crates/ecs/src/systems`。
- 当前 Bevy 版本使用 `Message` / `add_message` 作为事件通道 API；`events` 目录表达 ECS 事件语义，不表示必须使用旧版 `Event` API。
- 事件类型注册写在 `events/mod.rs` 的 `EventsPlugin`。

## 当前模板

当前模板只保留 ECS 基础结构和基础组件命名，不携带默认内容资源。

- `crates/ecs/src/components/base`: 定义 `DisplayName`、`PublicEntityId`、`Health`、`MaxHealth`、`Speed`、`Velocity2d`、`Velocity3d`、`MovementIntent`、`MovementTarget`、`Facing`、`Faction`、`Team`、`AudioClip`、`AudioClips`。
- `crates/ecs/src/components/characters`: 定义 `Character` 等通用角色身份 marker。
- `crates/ecs/src/components/items`: 定义基础物品身份数据。
- `crates/ecs/src/components/ui`: 保留 UI 相关 ECS 数据命名空间。
- `crates/ecs/src/components/world`: 定义 gameplay-facing 实体标记，例如 `GameplayEntity`、`GameplaySessionEntity`、`GameplayEntityId`。
- `crates/ecs/src/resources`: 定义 `WorldConfig`、`GameSession`，并通过 `ResourcesPlugin` 注册默认 Resource。
- `crates/ecs/src/events`: 定义 `DamageEvent`、`HealEvent`、`SpawnedEvent`、`DiedEvent`，并通过 `EventsPlugin` 注册事件类型。
- `crates/intent`: 提供写入 `MovementIntent` 等意图数据的语义 API。
- `crates/gameplay`: 管理 Playing 状态下的 gameplay session 进入和系统调度。
- `crates/ecs/src/systems`: 放根据意图移动 `Transform` 的系统函数。

## 验证要求

修改 `crates/ecs` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
