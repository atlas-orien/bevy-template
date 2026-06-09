# input

`input` 是输入来源适配层。

它负责读取键盘、鼠标、手柄、网络消息、服务端推送、外设、脚本、回放等控制来源，并把这些来源转换成项目内部语义。

第一版只实现本地键盘移动输入，以及一个调用 `gameplay::api` 的窄转发入口。

核心规则：

```text
input 统一来源。
intent 统一持续性 Entity 行为。
gameplay::api 统一一次性高层玩法请求。
```

## 职责

- 读取外部输入来源。
- 把持续性角色行为转换成 `intent`。
- 把一次性高层玩法请求转换成 `gameplay::api`。
- 需要写入 intent 时，找到这些输入应该控制的 `Entity`。

## 当前结构

- `local`: 本地输入，例如键盘、鼠标、手柄。
- `gameplay_api`: 调用 gameplay API 的窄转发入口。

未来需要网络输入、服务端推送、回放输入时，再按需求添加新模块。不要提前维护空目录。

## 和 intent 的区别

`input` 关心输入来自哪里。

`intent` 关心某个 `Entity` 想做什么。

`gameplay::api` 关心外部希望 gameplay 做什么。

例如：

```text
input/local 读取 WASD
input/local 找到 LocalPlayerControlled Entity
input/local 调用 intent::movement::set_movement_intent
intent 给这个 Entity 写入 MovementIntent
ecs/systems/movement 执行移动规则
```

网络或外设触发一次性高层请求时：

```text
input/network 收到服务端消息
input/network 构造 gameplay::api::GameplayRequest
input/gameplay_api 提交请求
gameplay 内部 system 消费请求
prefab/state/world 发生变化
```

## 不应该放这里

- 不定义核心 ECS 数据。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不决定移动速度、路径、碰撞或到达判定。
- 不写渲染、动画、UI、相机。
- 不生成 prefab。
