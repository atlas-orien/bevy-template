# 2D 渲染子包

`render_2d` 是 2D 表现层。

它的任务是“把世界显示出来”，不是“决定世界怎么变化”。

## 职责

- 2D 相机。
- 2D 屏幕、抬头显示、菜单、界面。
- 精灵、纹理图集、2D 动画。
- 读取 `ecs` 数据，把游戏世界显示出来。
- 创建渲染专用 Entity、Component 和动画状态。

## 当前结构

- `camera`: 定义 2D 相机表现 bundle。
- `characters`: 角色 2D 表现命名空间。
- `screens`: 屏幕表现命名空间。
- `ui`: 2D UI 表现命名空间。

## 和 ecs/intent/gameplay 的区别

- `crates/ecs/src/components` 定义玩家是什么。
- `crates/ecs/src/systems` 定义玩家位置等 ECS 数据如何被规则改变。
- `input` 读取 local/device/AI 等控制来源；网络是双向通信层，v2 单独设计。
- `intent` 表达 Entity 想做什么。
- `prefab` 把 render_2d 的表现 bundle 组合进完整对象模板。
- `gameplay` 负责状态流、gameplay session 生命周期和 system 调度。
- `render_2d` 只负责玩家看起来是什么样。

## 渲染实体

表现层可以创建渲染专用子实体。

例如：

```text
Gameplay Entity
└── Render Entity
```

玩法 Entity 的 `Transform` 表示世界位置，由 ECS system 修改。

渲染子 Entity 的 `Sprite`、贴图图集、scale、动画 timer 由 `render_2d` 修改。

这样可以避免为了显示效果直接改玩法 Entity 的核心数据。

## 不应该放这里

- 不定义核心游戏数据。
- 不读取键盘来决定 Entity 意图。
- 不写入 intent。
- 不执行战斗、碰撞、物品结算等世界规则。
- 不依赖 input、intent、prefab、physics、gameplay。
- 不放 3D 网格、3D 灯光、3D 相机。
