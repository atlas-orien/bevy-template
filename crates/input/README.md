# input

`input` 是输入来源适配层。

它负责读取键盘、鼠标、手柄、外设、AI、脚本、回放等控制来源，并把这些来源转换成项目内部语义。

第一版只实现本地键盘移动输入，以及一个调用 `gameplay::api` 的窄转发入口。

核心规则：

```text
input 统一来源。
intent 统一持续性 Entity 行为。
gameplay::api 统一一次性高层玩法请求。
input::runtime 负责外部输入来源自己的 loop。
```

## 职责

- 读取外部输入来源。
- 把持续性角色行为转换成 `intent`。
- 把一次性高层玩法请求转换成 `gameplay::api`。
- 需要写入 intent 时，找到这些输入应该控制的 `Entity`。
- 为外设、AI、脚本、回放等 Bevy 外部来源提供 runtime loop。

## 当前结构

- `local`: 本地输入，例如键盘、鼠标、手柄。
- `device`: 外设输入。
- `ai`: AI 控制输入。
- `runtime`: input 自己的外部来源运行循环。
- `bridge`: input runtime 和 Bevy App 之间的通道。
- `gameplay_api`: 调用 gameplay API 的窄转发入口。

`input` 按来源域组织代码。来源域内部再继续按具体协议、设备或控制模型拆分。

网络不是 `input` 的子目录。网络是双向通信层，v2 需要单独设计 crate。

## 和 intent 的区别

`input` 关心输入来自哪里。

`intent` 关心某个 `Entity` 想做什么。

`gameplay::api` 关心外部希望 gameplay 做什么。

例如：

```text
input/local/keyboard 读取 WASD
input/local/keyboard 找到 LocalPlayerControlled Entity
input/local/keyboard 调用 intent::movement::set_movement_intent
intent 给这个 Entity 写入 MovementIntent
ecs/systems/movement 执行移动规则
```

外设或 AI 触发一次性高层请求时：

```text
input/device 收到外设信号
input/device 构造 gameplay::api::GameplayRequest
input/gameplay_api 提交请求
gameplay 内部 system 消费请求
prefab/state/world 发生变化
```

## runtime

`input::runtime` 是 input 自己的 loop/runner。

它适合运行：

```text
device
ai
```

它不应该直接操作 Bevy `World`。

正确路径是：

```text
input runtime
-> bridge/channel
-> Bevy App 内部消费 bridge
-> intent 或 gameplay::api
```

## 不应该放这里

- 不定义核心 ECS 数据。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不决定移动速度、路径、碰撞或到达判定。
- 不写渲染、动画、UI、相机。
- 不生成 prefab。
- 不定义 `InputPlugin`，不作为 Bevy plugin 注册到 app。
