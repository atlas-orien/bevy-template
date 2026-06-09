# components

`components` 放挂在 Entity 上的 ECS 数据。

这里的数据只描述“实体拥有什么”，不执行行为。真正读取和修改这些数据的系统函数，放到 `crates/ecs/src/systems`。

## 分类

### base

`base` 放最基础、最小颗粒度、可复用的组件。

文件规则：

- 按最小语义组拆文件，不按“一类型一文件”机械拆分。
- 一个文件只放同一语义组的基础组件和强绑定辅助类型。
- `identity.rs`: `DisplayName`、`PublicEntityId`。
- `health.rs`: `Health`、`MaxHealth`。
- `movement.rs`: `Speed`、`Velocity2d`、`Velocity3d`、`MovementIntent`、`MovementTarget`、`Facing`。
- `affiliation.rs`: `Faction`、`Team`。
- `ecs` 不按 2D / 3D 创建目录。2D / 3D 只是同一语义组里的数据形状变体，例如 `Velocity2d` 和 `Velocity3d` 都放在 `movement.rs`。
- 如果 2D / 3D 差异属于渲染、物理后端或 prefab 组合，放到 `render_2d`、`render_3d`、`physics` 或 `prefab/world_2d`、`prefab/world_3d`。
- 不要把无关基础组件混在一个文件里，例如不要在 `movement.rs` 写血量、攻击、背包、AI 或 gameplay manager 数据。

适合放这里：

- 血量：`Health`、`MaxHealth`
- 移动：`Speed`、`Velocity2d`、`Velocity3d`、`MovementIntent`、`Facing`
- 身份：`DisplayName`、`PublicEntityId`
- 阵营：`Faction`、`Team`

不适合放这里：

- `Player`、`Enemy` 这种角色身份，放到 `characters`
- `Weapon`、`Consumable` 这种物品身份，放到 `items`
- 系统函数，例如 `movement_system`

### characters

`characters` 放角色类实体的数据定义和默认 Bundle。

适合放这里：

- 角色身份 marker：`Player`、`Enemy`、`Npc`
- 角色默认组合：`PlayerBundle`、`EnemyBundle`、`NpcBundle`
- 角色专属纯数据：例如职业、等级、角色类型

`characters` 可以组合 `base` 里的基础组件，但不写行为逻辑。

不适合放这里：

- 键盘、手柄、AI 控制逻辑
- 移动、伤害、死亡等系统函数
- sprite、动画、模型、相机、UI 创建

### items

`items` 放物品类实体的数据定义和默认 Bundle。

适合放这里：

- 物品身份：`Item`
- 武器：`Weapon`
- 防具：`Armor`
- 消耗品：`Consumable`
- 掉落物：`Pickup`
- 堆叠数量：`StackCount`

不适合放这里：

- 拾取系统
- 使用物品系统
- 装备系统
- 物品图标渲染

### ui

`ui` 放 UI 相关 ECS 数据。

适合放这里：

- HUD 根节点 marker：`HudRoot`
- 菜单 marker：`MainMenuRoot`、`PauseMenuRoot`
- 血条数据：`HealthBar`
- 名字牌数据：`Nameplate`
- 按钮动作：`ButtonAction`
- UI 和实体的绑定关系：例如血条绑定到哪个角色

不适合放这里：

- 创建 UI 节点的系统
- UI 布局、颜色、字体、图片
- 按钮点击后的业务逻辑
- 2D/3D 具体显示方式

### world

`world` 放世界、地图、关卡相关 ECS 数据。

适合放这里：

- 世界配置：`WorldConfig`
- 玩法实体标记：`GameplayEntity`
- 关卡 marker：`LevelMarker`
- 出生点：`SpawnPoint`
- 区域：`Zone`
- 障碍物：`Obstacle`
- 传送点：`Portal`

不适合放这里：

- 关卡加载系统
- 出生实体系统
- 碰撞和寻路逻辑
- 背景图片或地图渲染

## 判断规则

如果它是挂在某个实体上的数据，放到 `crates/ecs/src/components`。

如果它是全局数据，放到 `crates/ecs/src/resources`。

如果它是系统之间传递的消息，放到 `crates/ecs/src/events`。

如果它会读取数据并改变世界，放到 `crates/ecs/src/systems`。
