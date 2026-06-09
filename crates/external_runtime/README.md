# external_runtime

`external_runtime` 是 Bevy App 外部的 runtime 和 manager-side adapter 层。

项目运行时有两套系统：

```text
Bevy App
External Runtime
```

Bevy App 负责 `World`、`Schedule`、render、physics、gameplay。

External Runtime 负责 Bevy App 外部的来源模块，例如 input/local、input/device、input/ai、script、replay，以及未来 v2 单独设计的 network。

## 职责

- 启动和停止 Bevy App 外部的 runtime loop。
- 持有 `external_runtime::manager::ExternalRuntimeManager`。
- 运行 input/local、input/device、input/ai、script/replay 等外部来源模块。
- 把外部来源转换成 manager API 调用。
- 不直接操作 Bevy `World`。

## 当前结构

- `runtime`: external runtime 的 loop/runner。
- `manager`: manager API，分为用户 API 和 gameplay bridge API，不暴露 Bevy `Entity`。
- `bridge`: external runtime 和 Bevy App/gameplay manager 之间的桥接。
- `input`: 输入来源域。
- `input/local`: 本地输入来源，例如键盘、鼠标、手柄。
- `input/device`: 外设输入来源。
- `input/ai`: AI 输入来源。

网络不是 v1 子模块。网络是双向通信层，v2 单独设计。

## runtime

`external_runtime::runtime` 是 Bevy App 外部的 loop/runner。

正确路径是：

```text
external source
-> external_runtime
-> ExternalRuntimeManager
-> GameplayBridgeApi
-> Bevy App inbox
-> gameplay request systems
```

## 不应该放这里

- 不定义核心 ECS 数据。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不决定移动速度、路径、碰撞或到达判定。
- 不写渲染、动画、UI、相机。
- 不生成 prefab。
- 不定义 Bevy `Plugin`，不作为 Bevy plugin 注册到 app。
