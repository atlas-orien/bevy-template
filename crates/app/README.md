# 应用子包

`app` 是最终运行入口子包，负责组装空的 Bevy 应用外壳。

## 职责

- 配置 `DefaultPlugins`、窗口等顶层 Bevy 设置。
- 配置 Bevy 顶层运行环境。
- 保持模板默认运行入口干净，不注册 demo gameplay。

## 当前默认组装

```rust
app::run()
```

默认 `app` 只启动一个空窗口。demo 游戏流程、demo 菜单、prefab 展示等开发期内容放在 `crates/dev_preview`。

运行完整 demo：

```sh
cargo run -p dev_preview
```

指定某个预览：

```sh
cargo run -p dev_preview -- demo_game
cargo run -p dev_preview -- demo_menu
```

`external_runtime` 不作为 Bevy plugin 注册到 app。AI、脚本、回放等 Bevy App 外部来源由 external runtime 持有 `ExternalRuntimeManager` 进入 gameplay；默认空 app 不启动这些 demo 通道。

本机键盘、鼠标和手柄由 `peripherals` 作为 Bevy App 内部 plugin 接入。UI 和世界对象 hover/click 等 Bevy interaction 由 `interaction` 作为 Bevy App 内部 plugin 接入。默认空 app 不注册它们；demo preview 需要时自行组装。

## 不应该放这里

- 不写具体组件。
- 不写具体输入控制逻辑。
- 不写移动、战斗、生成等模拟逻辑。
- 不写具体精灵、网格、界面细节。
- 不默认注册 demo gameplay、demo input 或 demo interaction。

这里应该保持很薄，只做 Bevy 外壳。需要演示完整 gameplay 时使用 `dev_preview`。
