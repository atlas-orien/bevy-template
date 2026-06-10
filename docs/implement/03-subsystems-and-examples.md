# 步骤 3：子系统与示例填充

前两步打通了「输入 → 玩法 → 表现」的主干。这一步把脚手架状态的子系统填成真实现，并补齐示例。优先级低于步骤 1、2，可按需挑选。

每个子任务都是独立的，做之前先读对应 `AI_PROTOCOL/*.md`。

## 3a. render_2d 表现层

- 现状：`camera / screens / characters / ui` 等已有插件骨架，但大量 `example.rs` 只是占位 marker（如 `ExampleCamera2d`）。
- 要做：把 `Player2dPrefab` 用到的渲染（`render_2d::characters::example::ExampleCharacter2dBundle`）作为范例，逐步补 sprite、动画帧、UI 等真实表现。
- 边界：`render_2d` 只读表现状态，**不驱动玩法规则**，不读输入，不依赖 `physics/prefab/intent`（见 `AI_PROTOCOL/RENDER_2D.md` 与 xtask 规则）。

## 3b. animation

- 现状：`animation/frame`、`animation/skeletal` 有结构与示例占位。
- 要做：实现帧动画驱动（按 `Facing`、移动状态切帧），先覆盖默认玩家。

## 3c. audio

- 现状：`crates/audio` 已有 sample/procedural/bus/playback/volume 结构。
- 要做：补一个可被 gameplay 触发的最小播放路径（如移动/碰撞音效），通过既有请求/事件机制接入，不要让 gameplay 直接依赖 audio 后端。

## 3d. render_3d

- 现状：`Scene3dPlugin / Camera3dPlugin / Ui3dPlugin` 为占位插件，默认 app 未接入。
- 要做：保持独立，仅在确有 3D 需求时再由上层接入；本步骤可只补占位实现，不强行接进默认 app。

## 3e. 黄金路径示例（重要）

- 现状：缺一条带注释、可模仿的端到端纵切示例。
- 要做：以「默认玩家：输入源 → SetMovementIntent → movement_system → render」为主线，在注释或 `docs/` 里串成一条「做新功能照这个抄」的范例，并在 `AGENTS.md` 指向它。
- 价值：约束（xtask + AI_PROTOCOL）告诉 AI 不能做什么，这条示例告诉 AI 该怎么做，是当前最缺的一块。

## 验收

每个子任务按 README 通用验收全绿。新增/调整目录结构时，同步更新对应 `AI_PROTOCOL/*.md` 与 xtask 规则，保证 `cargo run -p xtask -- check` 通过。
