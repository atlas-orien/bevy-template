此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# PERIPHERALS

这个文件是 `crates/peripherals` 的 AI 规则。

`peripherals` 是 Bevy App 内部的本机外设适配层。

它不替代 Bevy 的输入系统。Bevy/winit 已经负责接收键盘、鼠标和手柄；`peripherals` 只把这些 Bevy 原始输入转换成项目语义动作。

## 核心职责

- 读取 Bevy App 内部的本机外设输入，例如键盘、鼠标和手柄。
- 把设备细节转换成项目语义动作，例如移动、交互、UI action 或 outbound network action。
- 输入适配必须区分“物理输入”和“语义动作”。`KeyCode`、`MouseButton`、gamepad button 只是设备事实；`LocalInputAction` 才表示项目语义。
- 同一个物理输入在不同 `LocalInputContext` 下可以有不同含义，例如菜单中的方向键是 UI 焦点移动，角色场景中的方向键或 WASD 可以是移动意图。
- 键盘方向键和 Enter 这类 UI 操作要转换成 `interaction::UiNavigationInputMessage`，不要把 `KeyCode` 传给 `gameplay`。
- 作为 Bevy `Plugin` 注册到 App。
- 不直接读取或修改底层 ECS 结果。
- 不处理 protobuf、socket、远端连接、AI、脚本或回放。

## 代码落点

- 本机输入上下文和语义动作：写到 `crates/peripherals/src/local_input.rs`。
- 键盘适配：写到 `crates/peripherals/src/keyboard`。
- 键盘按键绑定按功能拆到 `crates/peripherals/src/keyboard/bindings/*.rs`，例如 `ui.rs`、`gameplay.rs`、未来的 `inventory.rs`、`dialog.rs` 或 `debug.rs`。
- 鼠标适配：写到 `crates/peripherals/src/mouse`。
- 手柄适配：写到 `crates/peripherals/src/gamepad`。

## 边界规则

- `peripherals` 可以读取 Bevy 输入资源，例如 `ButtonInput<KeyCode>`、`ButtonInput<MouseButton>` 和 gamepad 输入。
- `peripherals` 可以写入 interaction crate 定义的语义 message，例如 `UiNavigationInputMessage`。
- `peripherals` 可以定义普通 Rust 语义类型，例如 `LocalInputContext`、`LocalInputAction` 和本机 key binding 表；不要把这些类型派生成 Bevy `Resource`、`Event`、`Component` 或 `Bundle`。
- 新增键盘功能时，优先新增或扩展 `keyboard/bindings/{feature}.rs`，再在路由层把 `LocalInputAction` 投递到 interaction、gameplay 或 intent 边界；不要在一个系统里散写大量 `KeyCode` 判断。
- `peripherals` 不拥有游戏流程状态。当前是菜单、角色场景、背包、对话还是文本输入，应由 gameplay/app 侧通过明确边界决定，再由 `peripherals` 按上下文解释输入。
- `peripherals` 不直接使用 `Commands` 生成 gameplay entity。
- `peripherals` 不直接修改 `Transform`、速度、生命值、背包或物理组件。
- `peripherals` 不定义核心 `Component`、`Bundle`、`Resource`、`Event`。
- `peripherals` 不依赖 `ecs`、`physics`、`prefab`、`render_2d`、`render_3d` 或 `external_runtime`。
- `peripherals` 不构造 protobuf，不直接发送网络消息。
- `peripherals` 不承担 Bevy interaction、AI、脚本、回放或网络来源适配。
- 设备细节不能进入 `gameplay`、`intent`、`ecs`、`prefab` 或 render crate。

## 依赖规则

- `peripherals` 可以依赖 `bevy`。
- `peripherals` 可以依赖 `gameplay`，用于提交 Bevy App 内部的语义 gameplay 请求。
- `peripherals` 可以依赖 `interaction`，用于发布 UI 导航等本机交互语义 message。
- `peripherals` 可以依赖 `intent`，用于共享 intent 语义类型。
- `peripherals` 必须依赖 `error`。
- `peripherals` 不依赖 `external_runtime`。

## 验证要求

修改 `crates/peripherals` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
