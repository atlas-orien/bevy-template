# Bevy Template

一个为 AI 辅助开发准备的 Bevy 游戏 workspace 模板。

这个模板的目标不是发布到 crates.io，而是作为 GitHub 模板或本地 workspace 使用。项目默认先实现 2D 游戏结构，同时预留独立的 3D 渲染层。

## 设计目标

- 使用 Cargo workspace 拆分职责。
- 让 AI 后续开发时有明确的落点，不把 ECS、玩法、渲染和 app 组装混在一起。
- 默认 app 使用 2D 渲染层。
- 3D 渲染层独立存在，需要时再接入 app。
- 全项目统一使用 `error::Result<T>` 和 `error::GameError`。
- `error::GameError` 使用 `thiserror` 定义，外部错误转换统一在 `crates/error` 中添加。
- 所有 crate 都设置 `publish = false`，只从 GitHub 或本地路径加载。

## Workspace 结构

- `crates/error`: shared error event, severity, and logging system
- `crates/ecs`: ECS/domain 数据层，按角色、背景、物品、世界等分类
- `crates/gameplay`: 玩法规则、状态流、生成/销毁、交互、战斗等系统
- `crates/render_2d`: 2D 渲染和表现层，包含 2D camera、screen、UI、sprite 等
- `crates/render_3d`: 3D 渲染和表现层，包含 3D camera、scene、3D UI 等
- `crates/app`: 最终运行的 app crate，负责组装插件
- `src/main.rs`: workspace 根入口，让 `cargo run` 可以直接运行
- `assets`: Bevy 运行时资源
- `docs`: 设计文档、AI task brief、开发决策
- `tools`: 本地辅助脚本

## 默认组装

当前默认 app 组装：

```rust
ErrorPlugin
EcsPlugin
GameplayPlugin
Render2dPlugin
```

`Render3dPlugin` 已经存在，但默认不接入 app。需要切换或扩展 3D 模板时，在 `crates/app` 中组装它。

## 分层规则

- `ecs` 只放组件、bundle、resource、domain 数据定义。
- `gameplay` 放真正的游戏规则和系统逻辑。
- `render_2d` 只放 2D 表现相关代码。
- `render_3d` 只放 3D 表现相关代码。
- `app` 只负责最终插件组装和窗口等顶层配置。
- 可失败的项目函数统一返回 `error::Result<T>`。
- 每个非 `error` crate 都会把它重新导出为本 crate 的 `Result`。
- 不要在功能 crate 里自己定义新的 Result alias。
- 不要给 crate 加 `game_` 前缀，这个仓库本身就是游戏模板。

## 常用命令

运行：

```sh
cargo run
```

检查：

```sh
cargo fmt --check
cargo check --workspace
```

格式化：

```sh
cargo fmt
```

## AI 开发约定

给 AI 分配任务时，优先说明目标属于哪一层：

- 新角色、新物品、新世界数据：优先改 `crates/ecs`
- 新玩法规则、新状态流、新生成逻辑：优先改 `crates/gameplay`
- 新 2D UI、HUD、sprite、tilemap：优先改 `crates/render_2d`
- 新 3D camera、scene、light、mesh：优先改 `crates/render_3d`
- app 启动、插件组合、窗口配置：改 `crates/app`

更详细的 AI 开发规则见 `AGENTS.md`。
