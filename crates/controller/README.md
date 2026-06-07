# 控制器子包

`controller` 是控制层。

## 职责

- 读取键盘、手柄、鼠标、AI、脚本等控制来源。
- 把控制来源转换成意图组件。
- 例如把 WASD 输入转换成 `MovementIntent`。

## 当前示例

- `player`: 读取 WASD 和方向键，更新玩家的 `MovementIntent` 和 `Facing`。

## 和 simulation 的区别

`controller` 只表达“想做什么”。

例如：

- 玩家想向左移动。
- AI 想攻击目标。
- 脚本想打开门。

真正修改 `Transform`、生命值、背包、世界状态的代码应该放到 `simulation`。

## 不应该放这里

- 不直接生成实体。
- 不直接播放动画。
- 不直接修改渲染组件。
- 尽量不直接改变世界结果，只写意图组件。
