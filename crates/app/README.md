# 应用子包

`app` 是最终运行入口子包，负责组装 Bevy 应用。

## 职责

- 配置 `DefaultPlugins`、窗口、图片采样等顶层 Bevy 设置。
- 注册项目插件。
- 决定当前运行 2D 还是 3D 表现层。

## 当前默认组装

```rust
ErrorPlugin
EcsPlugin
PhysicsPlugin
SimulationPlugin
InputPlugin
IntentPlugin
Render2dPlugin
```

`Render3dPlugin` 已存在，但默认不接入。需要做 3D 模板时，在这里替换或追加。

## 不应该放这里

- 不写具体组件。
- 不写输入控制。
- 不写移动、战斗、生成等模拟逻辑。
- 不写具体精灵、网格、界面细节。

这里应该保持很薄，只做最终组装。
