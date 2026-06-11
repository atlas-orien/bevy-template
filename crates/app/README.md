# 应用子包

`app` 是最终运行入口子包，负责组装 Bevy 应用。

## 职责

- 配置 `DefaultPlugins`、窗口、图片采样等顶层 Bevy 设置。
- 注册游戏玩法插件。
- 配置 Bevy 顶层运行环境。

## 当前默认组装

```rust
GameplayPlugin::new(runtime_requests.inbox(), manager_updates.sender())
```

`GameplayPlugin` 是游戏玩法入口，内部负责 gameplay 状态、spawn、API 消费和 intent 能力。

`external_runtime` 不作为 Bevy plugin 注册到 app。AI、脚本、回放等 Bevy App 外部来源由 external runtime 持有 `ExternalRuntimeManager` 进入 gameplay。网络是双向通信层，v2 单独设计。

本机键盘、鼠标、手柄和 UI interaction 由 `peripherals` 作为 Bevy App 内部 plugin 接入。

顶层 `main` 创建两个具体 channel：

```text
RuntimeRequestChannel: ExternalRuntimeManager -> Bevy App / GameplayPlugin
ManagerUpdateChannel: Bevy App / GameplayPlugin -> ExternalRuntimeManager
```

接收方定义语义并持有 inbox；另一侧只拿 sender。

## 不应该放这里

- 不写具体组件。
- 不写具体输入控制逻辑。
- 不写移动、战斗、生成等模拟逻辑。
- 不写具体精灵、网格、界面细节。

这里应该保持很薄，只做 Bevy 外壳和 gameplay 入口组装。
