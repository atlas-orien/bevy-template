# base

`base` 放最基础、最小颗粒度、可复用的 ECS 数据。

适合放这里：

- 血量：`Health`、`MaxHealth`
- 属性：`Strength`、`Agility`、`Intelligence`
- 移动：`Speed`、`Velocity`、`MovementIntent`
- 身份：`DisplayName`、`EntityId`
- 阵营：`Faction`、`Team`

不适合放这里：

- `Player`、`Enemy` 这种角色身份，放到 `characters`
- `Weapon`、`Consumable` 这种物品身份，放到 `items`
- 系统函数，例如 `movement_system`
