# input

`input` 是输入来源适配层。

它负责读取键盘、鼠标、手柄、网络消息、服务端推送等外部来源，并把这些来源转换成 `intent`。

第一版只实现本地键盘移动输入。

## 职责

- 读取外部输入来源。
- 找到这些输入应该控制的 `Entity`。
- 调用 `intent` crate，把输入转换成实体意图。

## 当前结构

- `local`: 本地输入，例如键盘、鼠标、手柄。

未来需要网络输入、服务端推送、回放输入时，再按需求添加新模块。不要提前维护空目录。

## 和 intent 的区别

`input` 关心输入来自哪里。

`intent` 关心某个 `Entity` 想做什么。

例如：

```text
input/local 读取 WASD
input/local 找到 LocalPlayerControlled Entity
input/local 调用 intent::movement::set_movement_intent
intent 给这个 Entity 写入 MovementIntent
ecs/systems/movement 执行移动规则
```

## 不应该放这里

- 不定义核心 ECS 数据。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不决定移动速度、路径、碰撞或到达判定。
- 不写渲染、动画、UI、相机。
- 不生成 prefab。
