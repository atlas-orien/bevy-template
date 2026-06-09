# xtask

`xtask` 是项目内部工具 crate，不参与游戏运行，也不会发布到 crates.io。

它用来执行开发期任务，例如架构检查、协议检查、模板生成、资源检查等。当前只实现了架构检查。

## 使用方式

在仓库根目录运行：

```sh
cargo run -p xtask -- check
```

查看帮助：

```sh
cargo run -p xtask -- help
```

## 当前命令

### check

检查项目结构是否符合当前模板规则。

当前会检查 `crates/app`：

- `crates/app` 必须存在。
- `app` 只允许依赖 `bevy`、`error` 和 `gameplay`。
- `app` 不允许直接注册 prefab、input、intent、ecs、physics 或 render 插件。

当前会检查 `crates/intent`：

- `crates/intent` 必须存在。
- `AI_PROTOCOL/INTENT.md` 必须存在。
- 必须有当前意图分类目录，例如 `movement`。
- `intent` 可以依赖 `prefab`，但不允许依赖 `ecs`、`physics`、`render_2d`、`render_3d`。
- `intent` 不允许定义 ECS 数据类型。
- `intent` 不允许直接读取键盘、鼠标、手柄等输入来源。
- `intent` 不允许直接使用 `Commands`、`Transform` 或物理组件。

当前会检查 `crates/input`：

- `crates/input` 必须存在。
- `AI_PROTOCOL/INPUT.md` 必须存在。
- 必须有当前本地输入模块 `local.rs`。
- `input` 可以依赖 `prefab`，但不允许依赖 `ecs`、`physics`、`render_2d`、`render_3d`、`gameplay`。
- `input` 不允许定义 ECS 数据类型。
- `input` 不允许直接使用 `Commands`、`Transform` 或物理组件。

当前会检查 `crates/error`：

- `crates/error` 必须存在。
- `AI_PROTOCOL/ERROR.md` 必须存在。
- `error` 不允许依赖 `bevy`。
- `error` 不允许定义 Bevy gameplay、ECS 或 message 类型。
- 每个 crate 都必须依赖 `error`。
- 除 `crates/error` 外，其它 crate 不允许定义自己的 `Result` 类型别名。
- 除 `crates/error` 外，其它 crate 不允许直接使用 `std::result::Result` 或 `core::result::Result`。

当前会检查 `crates/ecs`：

- `crates/ecs` 必须存在。
- `AI_PROTOCOL/ECS.md` 必须存在。
- 不允许恢复旧的 `crates/components`。
- 不允许恢复旧的 `crates/system`。
- `crates/ecs/src/components` 下只保留一个集中 README。
- `crates/ecs/src/components` 中不允许定义 `_system` 结尾的函数。
- `crates/ecs/src/resources` 中不允许定义 `Component`、`Bundle`、`Event`。
- `crates/ecs/src/events` 中不允许定义 `Component`、`Bundle`、`Resource`，也不允许定义 `_system` 结尾的函数。
- `crates/ecs/src/systems` 中不允许定义 `Component`、`Bundle`、`Resource`、`Event`。

这些检查使用 `syn` 解析 Rust AST，不只是文本搜索。

`check-architecture` 作为旧命令别名保留，也可以继续运行。

当前也会检查 `crates/physics`：

- `crates/physics` 必须存在。
- `AI_PROTOCOL/PHYSICS.md` 必须存在。
- 只有 `crates/physics/Cargo.toml` 可以依赖 Avian 或 Rapier。
- 除 `crates/physics` 外，其它 crate 不允许直接 import Avian 或 Rapier。

当前也会检查 `crates/prefab`：

- `crates/prefab` 必须存在。
- `AI_PROTOCOL/PREFAB.md` 必须存在。
- `prefab` 不允许依赖 `input`、`intent`、`gameplay`。
- `prefab` 不允许直接读取键盘、鼠标、手柄等输入。

当前也会检查 `crates/gameplay`：

- `crates/gameplay` 必须存在。
- `AI_PROTOCOL/GAMEPLAY.md` 必须存在。
- `gameplay` 可以依赖 `prefab`，但不允许依赖 `ecs`、`physics`、`render_2d`、`render_3d`。
- `gameplay` 不允许定义 ECS 数据类型。
- `gameplay` 不允许直接读取键盘、鼠标、手柄等输入。

当前也会检查 `crates/render_2d`：

- `crates/render_2d` 必须存在。
- `AI_PROTOCOL/RENDER_2D.md` 必须存在。
- 默认表现目录 `camera`、`characters`、`screens`、`ui` 必须存在。
- `render_2d` 不允许依赖 `input`、`intent`、`prefab`、`physics`、`render_3d`。
- `render_2d` 不允许直接读取键盘、鼠标、手柄等输入。
- `render_2d` 不允许调用 intent 写入函数或引用物理组件。

## 和 AI_PROTOCOL 的关系

`AI_PROTOCOL` 写规则，`xtask` 执行其中适合自动化的部分。

现在对应关系是：

```text
AI_PROTOCOL/INTENT.md
crates/xtask/src/rules/intent.rs
crates/app
crates/xtask/src/rules/app.rs
AI_PROTOCOL/INPUT.md
crates/xtask/src/rules/input.rs
AI_PROTOCOL/ERROR.md
crates/xtask/src/rules/error.rs
AI_PROTOCOL/ECS.md
crates/xtask/src/rules/ecs.rs
AI_PROTOCOL/PHYSICS.md
crates/xtask/src/rules/physics.rs
AI_PROTOCOL/PREFAB.md
crates/xtask/src/rules/prefab.rs
AI_PROTOCOL/GAMEPLAY.md
crates/xtask/src/rules/gameplay.rs
AI_PROTOCOL/RENDER_2D.md
crates/xtask/src/rules/render_2d.rs
```

以后如果新增 crate 规则，可以继续扩展：

```text
AI_PROTOCOL/INTENT.md      -> crates/xtask/src/rules/intent.rs
AI_PROTOCOL/INPUT.md       -> crates/xtask/src/rules/input.rs
AI_PROTOCOL/RENDER_2D.md   -> crates/xtask/src/rules/render_2d.rs
AI_PROTOCOL/APP.md         -> crates/xtask/src/rules/app.rs
```

## 扩展规则

新增规则时建议按 crate 拆分：

```text
crates/xtask/src/rules/
├── ecs.rs
├── input.rs
├── intent.rs
├── gameplay.rs
├── render_2d.rs
├── render_3d.rs
└── app.rs
```

每个规则模块负责检查对应 crate 的结构和边界。

如果规则只是检查路径是否存在，可以用普通文件系统检查。

如果规则需要判断 Rust 代码结构，优先用 AST 解析，不要只靠字符串搜索。
