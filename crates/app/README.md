# 应用子包

`app` 是最终运行入口子包，负责组装 Bevy 应用。

## 职责

- 配置 `DefaultPlugins`、窗口、图片采样等顶层 Bevy 设置。
- 注册游戏玩法插件。
- 配置 Bevy 顶层运行环境。

## 当前默认组装

```rust
GameplayPlugin::new(gameplay_inbox)
```

`GameplayPlugin` 是游戏玩法入口，内部负责 gameplay 状态、spawn、API 消费和 intent 能力。

`external_runtime` 不作为 Bevy plugin 注册到 app。外设、AI 等外部来源由 external runtime 持有 `GameplayManager` 进入 gameplay。网络是双向通信层，v2 单独设计。

顶层 `main` 创建 `GameplayManager` 和 inbox：

```text
external runtime -> GameplayManager
Bevy App -> GameplayPlugin::new(inbox)
```

## 不应该放这里

- 不写具体组件。
- 不写具体输入控制逻辑。
- 不写移动、战斗、生成等模拟逻辑。
- 不写具体精灵、网格、界面细节。

这里应该保持很薄，只做 Bevy 外壳和 gameplay 入口组装。
