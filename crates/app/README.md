# 应用子包

`app` 是最终运行入口子包，负责组装 Bevy 应用。

## 职责

- 配置 `DefaultPlugins`、窗口、图片采样等顶层 Bevy 设置。
- 注册唯一游戏玩法插件。

## 当前默认组装

```rust
GameplayPlugin
```

`GameplayPlugin` 是游戏唯一玩法入口，内部负责组装 prefab、input、intent 等游戏层插件。`app` 不直接注册这些内部插件。

## 不应该放这里

- 不写具体组件。
- 不写输入控制。
- 不写移动、战斗、生成等模拟逻辑。
- 不写具体精灵、网格、界面细节。

这里应该保持很薄，只做 Bevy 外壳和 gameplay 入口组装。
