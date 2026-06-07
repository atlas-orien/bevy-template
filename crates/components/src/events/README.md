# events

`events` 放 Bevy ECS 事件数据。

适合放这里：

- 伤害事件：`DamageEvent`
- 治疗事件：`HealEvent`
- 生命周期事件：`SpawnedEvent`、`DiedEvent`
- 物品事件：`ItemPickedEvent`、`ItemUsedEvent`

事件只描述“发生了什么”。

不适合放这里：

- 扣血逻辑
- 死亡处理逻辑
- 掉落物生成逻辑
- UI 提示和动画
