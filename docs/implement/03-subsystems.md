# 步骤 3：子系统填充

前两步打通了「输入 → 玩法 → 表现」的主干。这一步把脚手架状态的子系统填成真实实现。优先级低于步骤 1、2，可按需挑选。

每个子任务都是独立的，做之前先读对应 `AI_PROTOCOL/*.md`。

## 3a. render_2d 表现层

- 现状：`camera / screens / characters / ui` 等已有插件骨架，部分文件只是可替换的占位 marker。
- 要做：在具体项目里添加 prefab 使用的渲染表现，逐步补 sprite、动画帧、UI 等真实表现。
- 边界：`render_2d` 只读表现状态，**不驱动玩法规则**，不读输入，不依赖 `physics/prefab/intent`（见 `AI_PROTOCOL/RENDER_2D.md` 与 xtask 规则）。

## 3b. animation

- 现状：`animation/frame`、`animation/skeletal` 有结构与占位模块。
- 要做：实现帧动画驱动，具体如何按状态切帧由项目对象定义。

## 3c. audio

- 现状：`crates/audio` 已有 sample/procedural/bus/playback/volume 结构。
- 要做：补一个可被 gameplay 触发的最小播放路径（如移动/碰撞音效），通过既有请求/事件机制接入，不要让 gameplay 直接依赖 audio 后端。

## 3d. render_3d

- 现状：`Scene3dPlugin / Camera3dPlugin / Ui3dPlugin` 为占位插件，默认 app 未接入。
- 要做：保持独立，仅在确有 3D 需求时再由上层接入；本步骤可只补占位实现，不强行接进默认 app。

## 3e. 黄金路径规则（重要）

- 现状：需要一条端到端规则链路，说明请求如何从外部来源进入世界模拟。
- 要做：以「输入源 → SetMovementIntent → movement_system → render」为主线，在 `docs/` 里维护一条「做新功能按这个边界放置」的规则，并在 `AGENTS.md` 指向它。
- 价值：约束（xtask + AI_PROTOCOL）告诉 AI 不能做什么，黄金路径告诉 AI 每一层应该负责什么。

## 验收

每个子任务按 README 通用验收全绿。新增/调整目录结构时，同步更新对应 `AI_PROTOCOL/*.md` 与 xtask 规则，保证 `cargo run -p xtask -- check` 通过。
