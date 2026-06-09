# events

`events` 放 Bevy ECS 事件数据。

适合放这里：

- 伤害事件：`DamageEvent`
- 治疗事件：`HealEvent`
- 生命周期事件：`SpawnedEvent`、`DiedEvent`
- 物品事件：`ItemPickedEvent`、`ItemUsedEvent`

事件只描述“发生了什么”。

文件规则：

- 按最小语义组拆文件，不按“一类型一文件”机械拆分。
- 一个文件只放同一语义组的 Event。
- `combat.rs`: 战斗事件，例如 `DamageEvent`、`HealEvent`。
- `lifecycle.rs`: 生命周期事件，例如 `SpawnedEvent`、`DiedEvent`。
- Event 不处理后果，只携带系统之间需要传递的数据。
- 当前 Bevy 版本使用 `Message` / `add_message` 作为事件通道 API；本目录仍然按 ECS 事件语义命名和组织。
- 注册事件类型写在 `events/mod.rs` 的 `EventsPlugin`。

不适合放这里：

- 扣血逻辑
- 死亡处理逻辑
- 掉落物生成逻辑
- UI 提示和动画
