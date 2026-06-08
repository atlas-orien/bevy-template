# 2D 渲染子包

`render_2d` 是 2D 表现层。

它的任务是“把世界显示出来”，不是“决定世界怎么变化”。

## 职责

- 2D 相机。
- 2D 屏幕背景、抬头显示、菜单、界面。
- 精灵、纹理图集、2D 动画。
- 读取 `ecs` 和 `simulation` 状态，把游戏世界显示出来。
- 创建渲染专用 Entity、Component 和动画状态。

## 当前示例

- `characters/player.rs`: 给带 `Player` 组件的实体添加 2D 精灵，并根据 `MovementIntent` 播放跑步动画。
- `camera`: 在 Playing 状态生成 2D 相机。
- `screens`: 生成演示背景和地面色块。

## 和 ecs/intent/simulation 的区别

- `crates/ecs/src/components` 定义玩家是什么。
- `crates/ecs/src/systems` 定义玩家位置等 ECS 数据如何被规则改变。
- `input` 读取键盘、鼠标、手柄、网络等外部来源。
- `intent` 表达玩家想往哪走。
- `simulation` 负责状态流和系统调度。
- `render_2d` 只负责玩家看起来是什么样。

## 渲染实体

表现层可以创建渲染专用子实体。

例如玩家：

```text
Player gameplay Entity
└── PlayerSprite render Entity
```

玩法 Entity 的 `Transform` 表示世界位置，由 ECS system 修改。

渲染子 Entity 的 `Sprite`、贴图图集、scale、动画 timer 由 `render_2d` 修改。

这样可以避免为了显示效果直接改玩法 Entity 的核心数据。

## 不应该放这里

- 不定义核心游戏数据。
- 不读取键盘来决定玩家意图。
- 不写入 intent。
- 不执行战斗、碰撞、物品结算等世界规则。
- 不依赖 input、intent、prefab、physics。
- 不放 3D 网格、3D 灯光、3D 相机。
