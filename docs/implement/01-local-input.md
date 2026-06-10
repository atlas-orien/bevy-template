# 步骤 1：本地输入源（让玩家能动）

## 目标

让玩家按方向键/WASD 时，目标玩家（示例使用 `GameplayEntityId(1)`）移动。本地键盘必须走架构规定的路：**在 `external_runtime` 内读输入 → 发 `RuntimeRequest::SetMovementIntent` → channel**，绝不在 Bevy App 内直接写 intent。

## 现状

`crates/external_runtime/src/input/local/keyboard.rs` 已经是 Bevy App 外部的 OS 键盘轮询来源，不读取 `Res<ButtonInput<KeyCode>>`。

## 入口（已预留）

- 轮询点：`crates/external_runtime/src/runtime/task.rs` 的 `poll_external_sources(&manager)`，每 16ms 调用一次。
- 发请求：`crates/external_runtime/src/manager` 暴露的自由函数
  `set_movement_intent(&manager, id, target) -> bool`。
- 目标 id：示例目标是 `prefab::identity::GameplayEntityId(1)`；模板默认不生成玩家，具体项目或 example 分支需要自行生成带这个 id 的 prefab。
- 移动目标类型：`intent::movement::MovementTarget`，变体 `None / Direction(Vec2) / Position(Vec2)`。
- 这两个类型 `external_runtime` 已作为依赖引入，无需改依赖。

## 步骤

### 实现真正的本地键盘源

1. 重写 `crates/external_runtime/src/input/local/keyboard.rs`：不再是 Bevy system，而是一个**读 OS 键盘状态**的普通函数，返回当前 `MovementTarget`。
   - 读 OS 键盘需要一个不依赖窗口的输入库，例如在 `crates/external_runtime/Cargo.toml` 加 `device_query`。
   - 方向键/WASD 合成方向向量，无按键时返回 `MovementTarget::None`，否则 `Direction(dir.normalize_or_zero())`。
2. 在 `poll_external_sources` 中调用它，对 `GameplayEntityId(1)` 调 `set_movement_intent`。
3. `input/local/mod.rs` 导出外部键盘来源类型。

## 已知注意点

- `device_query` 读的是全局键盘，**不分窗口焦点**（窗口失焦时按键仍生效）。对模板可接受，加注释说明即可。
- macOS 上全局读键盘可能需要「辅助功能」权限。若调试受阻，先用 1a 的脚本源验证管线，键盘读取作为可选增强。
- 若以后要焦点正确的本地输入，再考虑「app 内薄转发 shim：抓 winit 原始事件转发出 channel」的方案，但那是后续优化，不在本步骤。

## 落点约束

- 只动 `crates/external_runtime/`（`input/local/`、`runtime/task.rs`、`Cargo.toml`）。
- 不在 `gameplay`/`app` 注册任何键盘 system。
- 不让 `external_runtime` 读 `Res<ButtonInput>` 或任何 Bevy 输入资源。

## 验收

按 README 通用验收全绿。具体项目或 example 分支生成目标 prefab 后，`cargo run` 后用键盘能让该对象移动。
