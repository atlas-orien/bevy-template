# 步骤 3：子系统填充

前两步打通了「输入 → 玩法 → 表现」的主干。这一步把脚手架状态的子系统填成真实实现。优先级低于步骤 1、2，可按需挑选。

每个子任务都是独立的。正式修改代码时，以对应 `AI_PROTOCOL/*.md` 和 `xtask` 规则为准。

## 3a. render_2d 表现层

- 现状：`camera / screens / characters / ui` 等已有插件骨架，部分文件只是可替换的占位 marker。
- 要做：在具体项目里添加 prefab 使用的渲染表现，逐步补 sprite、动画帧、UI 等真实表现。
- 说明：当前 `render_2d` 只读表现状态，不驱动玩法规则，不读输入，不依赖 `physics/prefab/intent`。具体边界见 `AI_PROTOCOL/RENDER_2D.md` 与 xtask 规则。

## 3b. animation

- 现状：`animation/frame`、`animation/skeletal` 有结构与占位模块。
- 要做：实现帧动画驱动，具体如何按状态切帧由项目对象定义。

## 3c. audio

- 现状：`crates/audio` 已有 sample/procedural/bus/playback/volume 结构。
- 要做：补一个可被 gameplay 触发的最小播放路径（如移动/碰撞音效），通过既有请求/事件机制接入。当前架构不让 gameplay 直接依赖 audio 后端。

## 3d. render_3d

- 现状：`Scene3dPlugin / Camera3dPlugin / Ui3dPlugin` 为占位插件，默认 app 未接入。
- 要做：保持独立，仅在确有 3D 需求时再由上层接入；本步骤可只补占位实现，默认 app 可以继续保持 2D。

## 3e. 黄金路径说明（重要）

- 现状：需要一条端到端说明链路，解释请求如何从外部来源进入世界模拟。
- 要做：以「输入源 → SetMovementIntent → movement_system → render」为主线，在 `docs/` 里维护一条便于人理解的参考说明。
- 价值：`AI_PROTOCOL` 和 `xtask` 是硬约束；黄金路径只是解释这些约束背后的标准链路。

## 验收

每个子任务可参考 README 通用验收。新增/调整目录结构时，对应的硬约束来源是 `AI_PROTOCOL/*.md` 与 xtask 规则，并以 `cargo run -p xtask -- check` 的结果为准。
