# 实现路线图

这个目录记录项目实现路线和人工协作备忘。

每个文件是一个**独立、可验收**的实现步骤，按编号顺序阅读。

注意：这里的内容是参考说明，不是项目硬约束来源。项目硬约束只来自：

- `AI_PROTOCOL/*.md`
- `crates/xtask/src/rules/*.rs`

## 已经做完的部分

核心管线已经端到端打通。后续实现通常沿用它预留的入口补能力：

```
外部源 → manager::set_user_movement_intent(user, target) → RuntimeRequestChannel
      → forward_manager_requests_system          （gameplay/api/systems.rs）
      → consume_gameplay_requests_system 消费 SetMovementIntent
      → 按 GameplayEntityId 找 Entity → intent 写入 MovementIntent
      → movement_system 改 Transform              （ecs/systems/movement）
      → render_2d 渲染、physics 步进
```

已就绪的事实：

- `App` 启动后 `AppState::Loading` 会自动切到 `Playing`，`OnEnter(Playing)` 执行默认 spawn plan；模板默认 plan 为空，具体项目自行添加 prefab。
- `RuntimeRequestMessage` 已支持 `SpawnPrefab / DespawnEntity / ClearSession / ChangeState / SetMovementIntent`，并已在 `gameplay/api/systems.rs` 全部消费。
- `external_runtime` 已有独立 tokio 循环，每 16ms 调一次 `poll_external_sources(&manager)`（`crates/external_runtime/src/runtime/task.rs`），用于 AI、脚本、回放等 Bevy App 外部来源。
- manager 已暴露面向 runtime id 的自由函数，例如 `spawn_prefab_for_user / spawn_prefab_for_object / clear_session / change_state / set_user_movement_intent / set_object_movement_intent`（`crates/external_runtime/src/manager`）。
- `peripherals` 是 Bevy App 内部 plugin，用于本机键盘、鼠标、手柄和 UI interaction 适配。

## 核心架构决策记录

**来源按所在运行环境分层。** 本机键盘、鼠标、手柄和 UI interaction 属于 Bevy App 内部的 `peripherals`；AI、脚本、回放和未来网络属于 Bevy App 外部的 `external_runtime`。两边都先转成项目语义动作，再进入 gameplay 边界。

本机键盘/鼠标由操作系统经窗口送达，而窗口事件循环属于 bevy/winit，因此本机外设适配放在 `peripherals`，不在 `external_runtime` 里轮询 OS 设备状态。

## 步骤总览

1. `01-local-input.md` — 记录本机键盘输入适配方向。
2. `02-ai-control.md` — 实现 AI 控制源（本模板的立项目的）。
3. `03-subsystems.md` — 填充 audio / render_3d / animation / ui 子系统与表现占位模块。

网络输入属于 v2，结构上和 AI 源走同一条 `RuntimeRequestMessage` 路，本路线图暂不展开。

## 每步通用验收

参考验收命令：

```sh
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- check
```
