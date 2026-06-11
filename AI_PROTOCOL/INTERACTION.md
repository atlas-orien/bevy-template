# INTERACTION

这个文件是 `crates/interaction` 的 AI 规则。

`interaction` 是 Bevy App 内部的交互事件桥接层。

它不负责对象怎么显示，也不负责具体业务执行。Bevy UI 和 picking 负责命中测试与交互状态；`interaction` 把 UI、2D/3D 世界对象等 Entity 上的交互转换成统一的项目语义 message。

## 核心职责

- 读取 Bevy 交互状态，例如 UI `Interaction::Pressed` / `Hovered`。
- 未来可以接入 Bevy picking 的 click、hover、drag 等 pointer events。
- 读取 Entity 上的交互语义数据，发出统一 interaction message。
- 不生成具体 UI、怪物、道具或世界对象。
- 不执行具体 gameplay 业务。
- 不处理键盘、鼠标、手柄等硬件输入。
- 不处理 protobuf、socket、远端连接、AI、脚本或回放。

## 代码落点

- 交互语义数据：写到 `crates/interaction/src/action.rs`。
- 交互事件 message：写到 `crates/interaction/src/message.rs`。
- UI Button interaction 桥接：写到 `crates/interaction/src/ui.rs`。

## 边界规则

- `interaction` 可以读取 Bevy UI 的 `Button` 和 `Interaction`。
- `interaction` 可以定义交互语义 `Component`，例如 `InteractionAction`。
- `interaction` 可以定义交互 message，例如 `InteractionEventMessage`。
- `interaction` 不定义核心玩法 Bundle、Resource 或底层 ECS system。
- `interaction` 不直接修改 `Transform`、速度、生命值、背包或物理组件。
- `interaction` 不生成 gameplay entity。
- `interaction` 不依赖 `prefab`、`physics`、`render_2d`、`render_3d` 或 `external_runtime`。
- `interaction` 不构造 protobuf，不直接发送网络消息。

## 依赖规则

- `interaction` 可以依赖 `bevy`。
- `interaction` 必须依赖 `error`。
- `interaction` 不依赖 `external_runtime`。

## 验证要求

修改 `crates/interaction` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
