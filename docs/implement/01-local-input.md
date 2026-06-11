# 步骤 1：本地输入源

## 目标

方向键/WASD 产生移动意图请求。当前架构把本地键盘放在外部输入源链路里：**在 `external_runtime` 内读输入 → 发 `RuntimeRequestMessage::SetMovementIntent` → channel**，而不是在 Bevy App 内直接写 intent。

## 现状

`crates/external_runtime/src/input/local/keyboard.rs` 已经是 Bevy App 外部的 OS 键盘轮询来源，不读取 `Res<ButtonInput<KeyCode>>`。

## 入口（已预留）

- 轮询点：`crates/external_runtime/src/runtime/task.rs` 的 `poll_external_sources(&manager)`，每 16ms 调用一次。
- 发请求：`crates/external_runtime/src/manager` 暴露的自由函数
  `set_movement_intent(&manager, id, target) -> bool`。
- 目标 id：模板默认不生成对象；具体项目需要自行决定目标 `GameplayEntityId`，并生成带这个 id 的 prefab。
- 移动目标类型：`intent::movement::MovementTarget`，变体 `None / Direction(Vec2) / Position(Vec2)`。
- 这两个类型 `external_runtime` 已作为依赖引入，无需改依赖。

## 步骤

### 实现真正的本地键盘源

1. 重写 `crates/external_runtime/src/input/local/keyboard.rs`：不再是 Bevy system，而是一个**读 OS 键盘状态**的普通函数，返回当前 `MovementTarget`。
   - 读 OS 键盘可使用不依赖窗口的输入库，例如在 `crates/external_runtime/Cargo.toml` 加 `device_query`。
   - 方向键/WASD 合成方向向量，无按键时返回 `MovementTarget::None`，否则 `Direction(dir.normalize_or_zero())`。
2. 在 `poll_external_sources` 中调用它，对项目选定的 `GameplayEntityId` 调 `set_movement_intent`。
3. `input/local/mod.rs` 导出外部键盘来源类型。

## 已知注意点

- `device_query` 读的是全局键盘，**不分窗口焦点**（窗口失焦时按键仍生效）。对模板可接受，加注释说明即可。
- macOS 上全局读键盘可能需要「辅助功能」权限。若调试受阻，先用 1a 的脚本源验证管线，键盘读取作为可选增强。
- 若以后要焦点正确的本地输入，再考虑「app 内薄转发 shim：抓 winit 原始事件转发出 channel」的方案，但那是后续优化，不在本步骤。

## 参考落点

- 主要变更通常集中在 `crates/external_runtime/`（`input/local/`、`runtime/task.rs`、`Cargo.toml`）。
- 当前链路没有在 `gameplay`/`app` 注册键盘 system。
- 当前链路没有让 `external_runtime` 读 `Res<ButtonInput>` 或任何 Bevy 输入资源。

## 验收

参考 README 通用验收。具体项目生成目标 prefab 后，`cargo run` 后用键盘能让该对象移动。
