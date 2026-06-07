# 模拟子包

`simulation` 是游戏世界模拟层。

## 职责

- 管理 app/game 状态流。
- 生成和销毁实体。
- 根据意图组件修改世界。
- 执行移动、战斗、交互、背包、任务、碰撞等规则。

## 当前结构

- `flow`: 定义 `AppState`，例如 `Loading`、`MainMenu`、`Playing`、`Paused`。
- `spawning`: 在 Playing 状态生成玩家实体。
- `movement`: 根据 `MovementIntent` 和速度修改 `Transform`。

## 和 controller 的区别

`controller` 写入“想做什么”。

`simulation` 决定“实际发生什么”。

例如：

- controller 写入 `MovementIntent { direction }`
- simulation 根据速度、时间、规则修改 `Transform`

## 不应该放这里

- 不加载精灵或模型。
- 不播放动画。
- 不直接读取键盘、手柄、鼠标。
- 不写 UI。
