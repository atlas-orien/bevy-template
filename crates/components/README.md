# components

`components` 是 ECS 数据定义层。

## 职责

- 定义 Bevy `Component`、`Bundle`、`Resource`、`Event`、标记组件。
- 按游戏数据概念分类，例如基础属性、角色、物品、世界、UI、事件、资源。
- 提供其他 crate 共享使用的数据结构。

## 当前结构

- `base`: 基础组件，例如血量、属性、移动、身份、阵营。
- `characters`: 角色类实体数据，例如 `Player`、`Enemy`、`Npc` 以及对应 Bundle。
- `items`: 物品类实体数据，例如武器、防具、消耗品、掉落物。
- `world`: 世界、地图、关卡、区域、出生点等数据。
- `ui`: UI 相关 ECS 数据，例如 HUD、血条、菜单、按钮动作。
- `events`: ECS 事件数据，例如伤害、治疗、死亡、拾取。
- `resources`: Bevy ECS 全局 `Resource` 数据，例如配置、当前关卡、运行会话。

## 不应该放这里

- 不读取键盘、手柄、鼠标。
- 不写 AI 控制。
- 不修改实体位置或生命值。
- 不加载图片、模型、音频。
- 不写渲染动画。

判断规则：如果代码只是描述“一个东西拥有什么数据”，放这里；如果代码会“做事情”，通常不放这里。

## 和 assets 的区别

`assets/` 是磁盘文件目录，放图片、音频、字体、地图文件等外部资源。

`components/src/resources/` 是 Rust 数据目录，放 `#[derive(Resource)]` 的 ECS 全局数据类型。
