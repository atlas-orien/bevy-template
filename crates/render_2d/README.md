# 2D 渲染子包

`render_2d` 是 2D 表现层。

## 职责

- 2D 相机。
- 2D 屏幕背景、抬头显示、菜单、界面。
- 精灵、纹理图集、2D 动画。
- 读取 `components` 和 `simulation` 状态，把游戏世界显示出来。

## 当前示例

- `characters/player.rs`: 给带 `Player` 组件的实体添加 2D 精灵，并根据 `MovementIntent` 播放跑步动画。
- `camera`: 在 Playing 状态生成 2D 相机。
- `screens`: 生成演示背景和地面色块。

## 和 components/controller/simulation 的区别

- `components` 定义玩家是什么。
- `controller` 决定玩家想往哪走。
- `simulation` 修改玩家位置。
- `render_2d` 只负责玩家看起来是什么样。

## 不应该放这里

- 不定义核心游戏数据。
- 不读取键盘来决定玩家意图。
- 不执行战斗、碰撞、物品结算等世界规则。
- 不放 3D 网格、3D 灯光、3D 相机。
