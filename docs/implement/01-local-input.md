# 步骤 1：本地输入源（让玩家能动）

## 目标

让玩家按方向键/WASD 时，默认玩家（`GameplayEntityId(1)`）移动。本地键盘必须走架构规定的路：**在 `external_runtime` 内读输入 → 发 `RuntimeRequest::SetMovementIntent` → channel**，绝不在 Bevy App 内直接写 intent。

## 现状（需要修正）

`crates/external_runtime/src/input/local/keyboard.rs` 当前是错的占位：

- 它是个 Bevy system，读 `Res<ButtonInput<KeyCode>>`——这是 Bevy App 内部资源，外部 tokio 线程拿不到。
- 它直接调 `set_movement_intent` 写 intent，**绕过了 channel**。

这两点都违反架构，需要替换。它当前没有被任何地方注册，删除/重写不影响编译。

## 入口（已预留）

- 轮询点：`crates/external_runtime/src/runtime/task.rs` 的 `poll_external_sources(&manager)`，每 16ms 调用一次。
- 发请求：`crates/external_runtime/src/manager` 暴露的自由函数
  `set_movement_intent(&manager, id, target) -> bool`。
- 目标 id：默认玩家是 `prefab::identity::GameplayEntityId(1)`。
- 移动目标类型：`intent::movement::MovementTarget`，变体 `None / Direction(Vec2) / Position(Vec2)`。
- 这两个类型 `external_runtime` 已作为依赖引入，无需改依赖。

## 步骤

### 1a. 先做最小烟雾测试（确认管线，不引新依赖）

在 `poll_external_sources` 里临时写一个「脚本源」：每次 tick 给 `GameplayEntityId(1)` 发一个固定方向，例如 `MovementTarget::Direction(Vec2::X)`。

```sh
cargo run
```

应看到：窗口出现蓝色方块并持续向右移动。看到移动 = 整条 channel→intent→渲染管线确认可用。确认后删除这个临时脚本源，进入 1b。

### 1b. 实现真正的本地键盘源

1. 重写 `crates/external_runtime/src/input/local/keyboard.rs`：不再是 Bevy system，而是一个**读 OS 键盘状态**的普通函数，返回当前 `MovementTarget`。
   - 读 OS 键盘需要一个不依赖窗口的输入库，例如在 `crates/external_runtime/Cargo.toml` 加 `device_query`。
   - 方向键/WASD 合成方向向量，无按键时返回 `MovementTarget::None`，否则 `Direction(dir.normalize_or_zero())`。
2. 在 `poll_external_sources` 中调用它，对 `GameplayEntityId(1)` 调 `set_movement_intent`。
3. 删除 `input/local/mod.rs` 里对旧 Bevy system 的 `pub use keyboard_movement_input_system`，改成导出新函数。

## 已知注意点

- `device_query` 读的是全局键盘，**不分窗口焦点**（窗口失焦时按键仍生效）。对模板可接受，加注释说明即可。
- macOS 上全局读键盘可能需要「辅助功能」权限。若调试受阻，先用 1a 的脚本源验证管线，键盘读取作为可选增强。
- 若以后要焦点正确的本地输入，再考虑「app 内薄转发 shim：抓 winit 原始事件转发出 channel」的方案，但那是后续优化，不在本步骤。

## 落点约束

- 只动 `crates/external_runtime/`（`input/local/`、`runtime/task.rs`、`Cargo.toml`）。
- 不在 `gameplay`/`app` 注册任何键盘 system。
- 不让 `external_runtime` 读 `Res<ButtonInput>` 或任何 Bevy 输入资源。

## 验收

按 README 通用验收全绿，且 `cargo run` 后用键盘能让方块移动（或 1a 脚本源能让方块自动移动）。
