# characters

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
