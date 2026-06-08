# Bevy 游戏模板

一个为 Bevy 游戏开发准备的工作区模板。

这个模板的目标不是发布到 crates.io，而是作为 GitHub 模板或本地工作区使用。项目默认先实现 2D 游戏结构，同时预留独立的 3D 渲染层。

## 设计目标

- 使用 Cargo 工作区拆分职责。
- 让后续开发有明确的代码落点，不把 ECS、玩法、渲染和应用组装混在一起。
- 默认 app 使用 2D 渲染层。
- 3D 渲染层独立存在，需要时再接入 app。
- 全项目统一使用 `error::Result<T>` 和 `error::GameError`。
- `error::GameError` 使用 `thiserror` 定义，外部错误转换统一在 `crates/error` 中添加。
- 所有子包都设置 `publish = false`，只从 GitHub 或本地路径加载。

## 工作区结构

这是模板默认结构，不是固定不变的最终架构。

实际项目可以根据游戏类型和团队习惯添加、修改或删除目录。比如某个游戏不需要 3D，可以长期不接入 `render_3d`；如果物品系统很复杂，也可以继续在 `crates/ecs/src/components/items` 或其他合适位置细分模块。

调整目录时需要保持一个原则：目录名称应该表达清楚职责，代码应该放在最容易理解和维护的位置。修改结构后，也应该同步更新相关文档，避免后续开发继续按照旧路径写代码。

- `crates/error`: 统一错误、Result、错误事件、严重级别和日志收集
- `crates/ecs`: Bevy ECS 核心层，包含组件、资源、事件和系统函数
- `crates/input`: 输入来源适配层，把键盘、鼠标、网络等外部来源转换成 intent
- `crates/intent`: Entity 意图层，表达可控制实体想做什么
- `crates/simulation`: 模拟层，负责状态流、scene 进入/退出调度和 ECS system 调度
- `crates/physics`: 物理引擎适配层，默认使用 Avian 2D，可通过 feature 切换到 Rapier 2D
- `crates/prefab`: 可生成对象模板基础库，组合 ECS、physics、render 数据
- `crates/scenes`: 场景装配层，把 prefab 组合成主菜单、关卡等具体场景
- `crates/render_2d`: 2D 渲染和表现层，包含 2D 相机、屏幕、界面、精灵等
- `crates/render_3d`: 3D 渲染和表现层，包含 3D 相机、场景、3D 界面等
- `crates/app`: 最终运行的应用子包，负责组装插件
- `src/main.rs`: 工作区根入口，让 `cargo run` 可以直接运行
- `assets`: Bevy 运行时资源目录，模板默认只保留空目录
- `docs`: 设计文档、AI 任务说明、开发决策
- `tools`: 本地辅助脚本

## 默认组装

当前默认 app 组装：

```rust
ErrorPlugin
EcsPlugin
PhysicsPlugin
SimulationPlugin
InputPlugin
IntentPlugin
Render2dPlugin
```

`Render3dPlugin` 已经存在，但默认不接入 app。需要切换或扩展 3D 模板时，在 `crates/app` 中组装它。

## crate 关系

下面是当前模板的默认依赖和协作关系：

```mermaid
flowchart TD
    main["src/main.rs"] --> app["crates/app"]

    app --> ecs["crates/ecs"]
    app --> physics["crates/physics"]
    app --> simulation["crates/simulation"]
    app --> input["crates/input"]
    app --> intent["crates/intent"]
    app --> render2d["crates/render_2d"]

    input --> ecs
    input --> intent
    input --> simulation

    intent --> ecs

    simulation --> ecs
    simulation --> scenes["crates/scenes"]

    scenes --> prefab["crates/prefab"]
    prefab --> ecs
    prefab --> physics
    prefab --> render2d

    render2d --> ecs
```

`app` 依赖 `physics` 是因为 Bevy 插件需要在最终应用里注册。这里的 `app -> physics` 只表示：

```rust
app.add_plugins(PhysicsPlugin)
```

它不表示 app 负责物理规则，也不表示 app 会直接创建刚体或碰撞体。物理对象的组合放在 `prefab`，场景对象选择放在 `scenes`，场景切换时机放在 `simulation`，物理后端适配放在 `physics`。

`error` 是全项目共享基础层。所有 crate 都使用 `error::Result<T>` 和 `error::GameError`，不要在其它 crate 里定义新的 `Result` 别名，也不要直接使用 Rust 默认的 `std::result::Result` 作为项目函数返回类型。

## 分层规则

下面是模板默认规则，可以按项目需求调整。调整时优先保持职责清晰，而不是机械保留所有目录。

- `crates/ecs/src/components` 放组件、bundle、marker 等挂在实体上的数据定义。
- `crates/ecs/src/resources` 放 Bevy ECS 全局 Resource 数据。
- `crates/ecs/src/events` 放 ECS 事件数据。
- `crates/ecs/src/systems` 放真正读取和修改 ECS 数据的系统函数。
- `input` 读取键盘、鼠标、手柄、网络等外部来源，并转换成 `intent`。
- `intent` 只表达哪个 Entity 想做什么，并提供写入意图数据的语义 API。
- `simulation` 负责状态流、阶段调度和 scene 进入/退出调度，可以组合 `crates/ecs/src/systems`。
- `physics` 对外提供统一物理 API，内部通过 feature 选择物理后端。
- `prefab` 负责组合 `ecs`、`physics`、`render_2d` 等数据，提供可直接生成的完整对象模板。
- `scenes` 负责绑定具体 prefab，组成主菜单、关卡、战斗场景等。
- `render_2d` 只放 2D 表现相关代码，可以创建相机、sprite、UI 和渲染专用子实体，但不能驱动 gameplay 规则。
- `render_3d` 只放 3D 表现相关代码。
- `app` 只负责最终插件组装和窗口等顶层配置。
- 可失败的项目函数统一返回 `error::Result<T>`。
- 每个非 `error` 子包都会把它重新导出为本子包的 `Result`。
- 不要在功能子包里自己定义新的 `Result` 别名。
- 不要给子包加 `game_` 前缀，这个仓库本身就是游戏模板。

## 常用命令

运行：

```sh
cargo run
```

检查：

```sh
cargo fmt --check
cargo check --workspace
cargo run -p xtask -- check
```

格式化：

```sh
cargo fmt
```

使用 Rapier 2D 物理后端：

```sh
cargo run --features physics/rapier2d
```

## 协作规则

如果使用 AI 代理辅助开发，可以阅读根目录 [AI_PROTOCOL](/Users/ancient/src/rust/bevy-template/AI_PROTOCOL) 下对应 crate 的规则文件。

约定：普通 `README.md` 写给人看；`AI_PROTOCOL` 目录下的大写文件写给 AI 看。
