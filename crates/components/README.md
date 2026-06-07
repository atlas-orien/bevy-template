# 组件子包

`components` 是 ECS 数据定义层。

## 职责

- 定义 Bevy `Component`、`Bundle`、`Resource`、标记组件。
- 按游戏概念分类数据，例如角色、物品、背景、世界。
- 提供其他子包共享使用的数据结构。

## 当前结构

- `characters`: 角色相关组件，例如 `Player`、`PlayerSpeed`、`MovementIntent`、`Facing`。
- `background`: 背景和环境层数据。
- `items`: 物品数据。
- `world`: 世界级配置和资源。

## 不应该放这里

- 不读取键盘、手柄、鼠标。
- 不写 AI 控制。
- 不修改实体位置或生命值。
- 不加载图片、模型、音频。
- 不写渲染动画。

判断规则：如果代码只是描述“一个东西拥有什么数据”，放这里；如果代码会“做事情”，通常不放这里。
