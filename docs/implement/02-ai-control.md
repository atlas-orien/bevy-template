# 步骤 2：AI 控制源（本模板的立项目的）

## 目标

让 AI 作为一个外部控制源，通过和键盘**完全相同**的 `RuntimeRequestMessage` 管道驱动游戏：移动、生成、销毁、切状态。证明「把游戏交给 AI 来玩/来做」的闭环成立。

## 现状

`crates/external_runtime/src/input/ai/mod.rs` 是 1 行占位（`// AI control adapters belong here.`）。管道已就绪，缺的只是「AI 决策 → RuntimeRequestMessage」这个生产者。

## 入口（与步骤 1 相同）

- 轮询点：`poll_external_sources(&manager)`（`runtime/task.rs`）。
- 可用的 manager 自由函数（`crates/external_runtime/src/manager`）：
  - `set_movement_intent(&manager, id, target)` — 控制移动。
  - `spawn_prefab(&manager, prefab)` — 运行中生成对象（prefab 须实现 `prefab::Prefab`）。
  - `despawn_entity(&manager, id)` / `clear_session(&manager)` / `change_state(&manager, state)`。
  - `entity_ids(&manager)` / `has_entity(&manager, id)` — 查询当前世界里存在哪些 gameplay id（AI 只看公开 id，不接触 Bevy `Entity`）。

## 步骤

1. 在 `crates/external_runtime/src/input/ai/mod.rs` 定义一个 AI 源类型，持有 AI 需要的内部状态（目标点、决策计时器等），并提供一个 `poll(&self, &ExternalRuntimeManager)`（或返回若干 `RuntimeRequestMessage` 的方法）。
2. 先实现一个**最小可观察策略**证明闭环，例如：
   - 用 `entity_ids` 读到玩家 id；
   - 周期性朝某个目标点发 `set_movement_intent(id, MovementTarget::Position(target))`；
   - 到达后换下一个目标点。
3. 在 `poll_external_sources` 中调用 AI 源的 poll。
4. 把「读什么决策、何时决策」与「怎么发请求」分清楚：决策逻辑写在 `input/ai`，发请求统一用 manager 自由函数；当前实现不在这里直接碰 channel 类型。

## 当前边界说明

- AI 源只产出 `RuntimeRequestMessage`，当前链路不会直接修改 Bevy `World`，也不接触 `Entity`、`Component`。
- `input/ai` 当前只承载控制源逻辑，不放渲染、物理、ECS 数据定义。
- 真正接入大模型/外部 AI 进程的传输细节，后续可放 `crates/external_runtime/src/bridge`；本步骤先用进程内策略打通闭环即可。

## 参考落点

- 主要变更通常集中在 `crates/external_runtime/`（`input/ai/`、`runtime/task.rs`，必要时 `bridge/`）。
- 具体边界以 `AI_PROTOCOL/EXTERNAL_RUNTIME.md` 和 xtask 规则为准。

## 验收

参考 README 通用验收，且 `cargo run` 后无需人工输入，AI 源能让玩家自动按策略移动（或执行 spawn/despawn 等可观察行为）。
