# 应用子包

`app` 是最终运行入口子包，负责组装 Bevy 应用。

## 职责

- 配置 `DefaultPlugins`、窗口、图片采样等顶层 Bevy 设置。
- 注册游戏玩法插件。
- 按产品形态注册可选外部 adapter，例如本地输入、网络、外设。

## 当前默认组装

```rust
GameplayPlugin
InputPlugin
```

`GameplayPlugin` 是游戏玩法入口，内部负责 gameplay 状态、spawn、API 消费和 intent 能力。

`InputPlugin` 是当前 demo 的本地输入 adapter。它不是 gameplay 的必要组成部分；没有本地输入时，外部系统仍然可以通过 `gameplay::api` 请求 gameplay 做事。

## 不应该放这里

- 不写具体组件。
- 不写具体输入控制逻辑；只选择注册哪些外部 adapter。
- 不写移动、战斗、生成等模拟逻辑。
- 不写具体精灵、网格、界面细节。

这里应该保持很薄，只做 Bevy 外壳、gameplay 入口和外部 adapter 组装。
