# external_runtime

`external_runtime` 是 Bevy App 外部的 runtime 和 manager-side adapter 层。

项目运行时有两套系统：

```text
Bevy App
External Runtime
```

Bevy App 负责 `World`、`Schedule`、render、physics、gameplay。

External Runtime 负责 Bevy App 外部的来源模块，例如 input/ai、script、replay，以及可选 network。

## 职责

- 启动和停止 Bevy App 外部的 runtime loop。
- 持有 `external_runtime::manager::ExternalRuntimeManager`。
- 运行 input/ai、script/replay 等外部来源模块。
- 把外部来源转换成 manager API 调用。
- 不直接操作 Bevy `World`。

## 当前结构

- `runtime`: external runtime 的 loop/runner。
- `manager`: 有状态的 manager，分为用户 API、内部 transport 和状态 registry，不暴露 Bevy `Entity`。
- `bridge`: external runtime 和 Bevy App/gameplay channel 之间的通道组装。
- `input`: 输入来源域。
- `input/ai`: AI 输入来源。
- `input/network`: 可选网络来源，未配置时单机游戏不会启动网络。
- `config`: 使用 `toolcraft-config` 读取 runtime/network 配置。

本机键盘、鼠标和手柄属于 `crates/peripherals`。UI 和世界对象 hover/click 等 Bevy interaction 属于 `crates/interaction`。它们都不属于 external runtime。

网络是可选来源。`external_runtime` 只启动和轮询 `network` crate 暴露的客户端连接，不在这里重写 UDP、protobuf 或 cmdproto transport。

## runtime

`external_runtime::runtime` 是 Bevy App 外部的 loop/runner。

正确路径是：

```text
external source
-> external_runtime
-> ExternalRuntimeManager
-> request channel
-> gameplay request systems
-> update channel
-> manager state registry
```

用户代码只使用 `ExternalRuntimeManager` 和 `manager/user.rs` 导出的函数。内部 transport 不对用户公开。

## 不应该放这里

- 不定义核心 ECS 数据。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不决定移动速度、路径、碰撞或到达判定。
- 不写渲染、动画、UI、相机。
- 不生成 prefab。
- 不定义 Bevy `Plugin`，不作为 Bevy plugin 注册到 app。
