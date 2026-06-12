# 06 - 质量收尾

这份文档处理 04 / 05 完成后遗留的全部质量缺口，目标把整体质量从 8/10 推到
9/10 并清零规则摩擦表。

注意：本文是实现路线说明，不是项目硬约束来源。项目硬约束只来自：

- `AI_PROTOCOL/*.md`
- `crates/xtask/src/rules/*.rs`

## 0. 目标与停止线

明确写下停止线：**9 分 + 摩擦表清零就停。** 最后一分是封装风格、命名偏好这类
口味问题，在 demo 代码上追求它的边际收益接近零。本文不包含任何"重写得更优雅"
类任务；每个阶段都对应一个已经在评审中点名的具体缺口。

缺口清单（按价值排序）：

| # | 缺口 | 对应阶段 |
|---|---|---|
| 1 | demo 玩法逻辑零测试，全靠人工运行验收 | C |
| 2 | session 生命周期跨 Paused/GameOver 是临时方案（摩擦记录 #1） | B |
| 3 | 摩擦表 3 条记录 + 4 个预见点未裁决，协议与现实不一致 | A |
| 4 | 流程尾巴：依赖检查旧名、readability 无测试、example.rs 占位残留 | D |
| 5 | 组件内部状态字段全 `pub`；模块级 `//!` 未强制 | E |

执行顺序：**A → B → C → D → E**。A 是人做决定（半小时），B 改架构（必须在 C
之前，否则测试写给临时方案）、C 是大头，D / E 收尾。A 完成前不要动 B。

每个阶段完成后统一跑：

```sh
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- check
```

---

## 阶段 A — 摩擦表裁决（人来做，AI 不动手）

这是唯一一个由人执行的阶段。`05-2d-demo.md` §16 积累的条目需要逐条裁决，
否则协议和真实代码长期不一致，AI 下次读协议时会困惑于"文档说不行但代码就是这么写的"。

按 AGENTS.md 的"规则保护"条款，下面每条的落地都需要人在指令里**明确说
"修改 AI_PROTOCOL"或"修改 xtask 规则"**。每条给出建议裁决，确认后照抄指令即可：

| 摩擦条目 | 建议裁决 | 给 AI 的指令模板 |
|---|---|---|
| #0：`LocalInputContext` 从 peripherals 搬到 gameplay::api | 已成事实，补协议 | "修改 AI_PROTOCOL/PERIPHERALS.md 和 GAMEPLAY.md：输入上下文枚举的落点改为 gameplay::api，peripherals 读取它" |
| #1：session 生命周期 | 走本文阶段 B（pause 改 SubStates） | 见阶段 B |
| #2：BGM 体积超 1MB | 接受现状或换音频，不改规则 | （可选）"把 assets/audio/demo_bgm.ogg 换成 <新文件>" |
| 预见 1：PREFAB.md"模板不携带默认内容" vs demo | 改协议为前缀豁免 | "修改 AI_PROTOCOL/PREFAB.md：默认内容禁令改为'非 demo_ 前缀的默认内容不允许；demo 内容必须可按 05 文档 §15 清单整体删除'" |
| 预见 2：BGM 触发链路过长（gameplay 不能依赖 audio） | **不放宽**。链路已实现且能跑，放宽依赖换来的简化不值得开这个口 | 无需指令，在摩擦表标记"裁决：维持现状" |
| 预见 3：physics 事件经 prefab 双重中转 | 维持现状，模式已被 `demo_sensor_bridge` 验证可用 | 在 AI_PROTOCOL/PREFAB.md 把"事件桥接 + re-export"明确写成标准模式（指令："修改 AI_PROTOCOL/PREFAB.md，补充事件桥接落点说明"） |
| 预见 4：`movement_system` 注册归属 | 按现实补文档 | "修改 AI_PROTOCOL/ECS.md 或 GAMEPLAY.md：写明 ecs system 的调度注册由 gameplay 负责"（按当前代码实际归属写） |
| （新增）intent 规则去掉了 `Query`（04 实现时的口径微调，未记录） | 追认 | 在摩擦表补记一行，标记"裁决：追认，Transform 路径检查已覆盖危险形态" |

**验收**：摩擦表每行多一列"裁决"且无空格；协议改动各自跑过
`cargo run -p xtask -- check`。

---

## 阶段 B — Session 生命周期定型（摩擦记录 #1 的正式解）

### B1. 问题回顾

`Paused` 目前是顶层 `AppState` 的一个变体。Playing → Paused 会触发
`OnExit(Playing)`，所以 demo 实现时被迫移除了那里的 session 清理，
导致 GameOver → 重开路径的清理靠 `spawn_initial` 的非空检查这类局部补丁。

### B2. 方案决策（推荐已选）

**推荐：把暂停改成 Bevy `SubStates`。** Bevy 0.17 原生支持：

```text
AppState: Loading / MainMenu / Playing / GameOver   ← Paused 从这里删除
PauseState（SubStates，source = AppState::Playing）: Running / Paused
```

理由：
- `OnExit(AppState::Playing)` 重新成为 session 清理的正确挂点——暂停不再离开
  Playing，只有真正退出场景（GameOver、回主菜单）才触发清理。补丁全部可删。
- 暂停/恢复变成 `PauseState` 内部切换，`run_if(in_state(PauseState::Running))`
  替代现在散落的 Playing 判断。
- 这是 Bevy 官方为这个场景设计的机制，不是自造抽象。

备选（显式 `ClearSession` 请求清理）被否的原因：把"何时清理"从状态机挪到请求
时序，每个进入路径都要记得发请求，漏发就是脏 session——靠纪律不靠结构。

### B3. 改动落点

1. `crates/gameplay/src/state/state_def.rs`：`AppState` 删 `Paused`，
   新增 `PauseState`（derive `SubStates`，`#[source(AppState = AppState::Playing)]`）。
2. `crates/gameplay/src/lib.rs`（或 plugin 组装处）：`add_sub_state::<PauseState>()`。
3. `crates/gameplay/src/control/`：`TogglePause` 改为切 `PauseState`；
   原来"Paused 时只响应 TogglePause"的旁路系统改用
   `run_if(in_state(PauseState::Paused))`。
4. `crates/gameplay/src/schedule/`：
   - 玩法系统的 `run_if(in_state(AppState::Playing))` 中，语义是"暂停要冻结"的
     改为叠加 `in_state(PauseState::Running)`。
   - `OnEnter/OnExit(PauseState::Paused)`：暂停菜单生成/清理 +
     `Time<Virtual>` pause/unpause（若 demo 已用）。
   - **恢复 `OnExit(AppState::Playing)` 的 session 清理**，删除
     `spawn_initial` 里的非空跳过补丁。
5. 菜单 Back 行为（`run_demo_menu_action` 里判断 Paused 的分支）改判
   `PauseState`。
6. `AI_PROTOCOL/GAMEPLAY.md` 同步：状态流图加 `PauseState`，写明
   "session 清理挂 OnExit(Playing)；暂停不离开 Playing"（人发起，归阶段 A 的
   指令流程）。
7. 摩擦表 #1 标记"裁决：SubStates，已实施"。

### B4. 验收

全套命令 + 手工路径测试，逐条确认：

- Playing 中 Escape：世界冻结、暂停菜单出现；再 Escape 恢复，**世界对象原样**。
- 暂停中 Back：恢复游戏（等价 Escape）。
- 感应区扣血到 0 → GameOver → Start 重开：世界重新生成且**实体数不翻倍**
  （log 抽查 spawn 数量），BGM 不叠放。
- GameOver → 重开 → 再暂停 → 再恢复：组合路径无残留菜单、无双相机。

---

## 阶段 C — demo 测试层（最大的一分）

### C1. 原则

demo 同时是 AI 的参照例子：**例子里没有测试，AI 以后写新功能也不会带测试。**
所以这一层的价值是双份的——既保护现有行为，又示范"测试长什么样、放在哪"。

测试策略分两档，不引入新测试框架：

- **纯逻辑测试**：不需要 Bevy App 的函数（索引计算、状态判断、数据换算），
  直接 `#[cfg(test)]` 单元测试，放在被测代码同文件底部（仓库已有先例：
  xtask 的 rules 测试）。
- **系统级测试**：需要 ECS 世界的系统，用 `App::new()` + 手工
  `app.update()` 驱动。Bevy 系统测试不需要窗口：直接
  `app.add_systems(Update, 被测系统)`、插入需要的资源和实体、update、断言。
  不加 `DefaultPlugins`，只插被测系统真正读的东西（`Time` 用
  `app.insert_resource(Time::<()>::default())` 加手动 advance，或改用
  `Time` mock 模式，实现时按 Bevy 0.17 实际 API 选）。

不追求覆盖率数字。下面清单覆盖"改坏了人眼难发现"的逻辑，写完即停。

### C2. 测试清单（按文件）

**`crates/gameplay/src/interaction/ui/demo_menu.rs`**（纯逻辑为主）
- `previous_demo_menu_index` / `next_demo_menu_index`：0 的环绕、末项环绕、
  中间项常规推进。
- `DemoMenuAction::from_id`：四个合法 id 各自命中；未知 id 返回 `None`。
- 注：`focused_demo_menu_index` 等耦合 Query 的辅助函数若难以直接测，
  允许把"纯索引计算"部分提炼成不带 Query 的私有函数再测——**这是本阶段唯一
  允许的顺手重构**，超出此范围的重构不做。

**`crates/render_2d/src/animation/frame/demo_frame_animation.rs`**（系统级）
- 单帧动画（first == last）：index 恒定不推进。
- 多帧推进：累计够 `frame_seconds` 后 index +1，elapsed 归零。
- 末帧回绕到 first_frame。
- `set_range` 切换范围时 elapsed 归零；范围相同时不归零（这是防抖语义，
  改坏了动画会每帧重置）。

**`crates/render_2d/src/particles/demo_particles.rs`**（系统级）
- 寿命衰减到 0 时实体被 despawn（update 后查询数量减少）。
- `initial_alpha` 淡出：剩余 50% 寿命时 alpha ≈ initial_alpha × 0.5。
- 发射上限：live 数达到 `max_live_particles` 时不再新增且 accumulator 归零。
- 发射速率：固定 delta 累计后生成数符合 particles_per_second。

**`crates/ecs/src/systems/movement/mod.rs`**（系统级，核心规则）
- `Direction`：按 speed × delta 位移；对角方向归一化（位移长度一致）。
- `Position`：到达 epsilon 内时 target 归 `None` 且停止。
- `Facing` 翻转：向左走变 Left，向右走变 Right，纵向移动不变。

**`crates/gameplay/src/control/`（阶段 B 完成后）**（系统级）
- `Move(dir)` 消息落到受控实体的 `MovementIntent`；零向量落成 `None`。
- `TogglePause` 在 `PauseState` 间正确切换。

**`crates/prefab/src/world_2d/demo_level/demo_sensor_bridge.rs`**（系统级）
- 带 `DemoSensorZone` 的 sensor 事件被转发；不带 marker 的被过滤。

**`crates/prefab/src/health/`**（纯逻辑）
- 扣血不下穿 0；到 0 的判定边界。

### C3. 落点与命名

- 测试与被测代码同文件，`#[cfg(test)] mod tests`，遵守仓库现有风格。
- 注意 400 行上限（xtask readability 规则）：加测试后超限的文件，
  把测试拆到同目录 `xxx_tests.rs` 是不行的（不是 mod 结构）——正确做法是
  把被测文件先按语义拆小，测试跟着各自的文件走。预计 `demo_particles.rs`
  （170 行 + 4 个系统的测试）会触线，可拆为 `dust.rs` / `burst.rs`。
- 系统级测试的公共脚手架（建最小 App、推进时间）若出现三次以上重复，
  提到该 crate 的 `#[cfg(test)]` 共享模块；不要建跨 crate 的 test util 包。

### C4. 验收

```sh
cargo test --workspace   # 新增测试全过
```

外加一次破坏性自检：随手反转 `demo_frame_animation` 的回绕条件、
`movement_system` 的归一化，确认对应测试变红（再恢复）。测试测不出注入的 bug
就是没写对。

---

## 阶段 D — 流程尾巴清理

四件机械工作，一次做完：

1. **依赖检查改名收尾**：11 处调用点 `reject_workspace_dependencies` →
   `reject_dependencies`，删除 `crates/xtask/src/rules/base/dependencies.rs`
   里的转发包装。名字撒谎比没有名字更糟。
2. **`readability.rs` 补测试**：三条规则各配最小用例——lib.rs 缺 `//!` 报错/
   有则通过；401 行文件报错；mod.rs 含 `fn` 报错、纯 `mod`+`use` 通过。
   用 `tempfile` 或内存构造？xtask 现有测试是纯函数式的——把检查函数里
   "读文件"和"判内容"拆开（判内容的部分接收 `&str`），只测判定逻辑，
   不引入临时文件依赖。
3. **`example.rs` 占位清理**：已有 `demo_*.rs` 真实实现的目录
   （tilemap、background、particles、camera、characters、props、overlays、
   animation/frame、atlases），删除同目录 `example.rs` 及其 `mod.rs` 导出。
   尚无真实实现的目录（lighting、mesh、transitions 等）保留占位。
   注意 xtask 的 render 规则可能锚定了某些 example 路径——删除时若被拦，
   按 AGENTS.md 流程报告，由人发指令调整规则（预计归阶段 A 的裁决流程）。
4. **`DEV_PREVIEW` 协议复审**（人做，15 分钟）：AI 自建的
   `AI_PROTOCOL/DEV_PREVIEW.md` + 对应 xtask 规则逐条过一遍，确认符合意图；
   不符合的条目由人发指令修改。

**验收**：全套命令绿；`grep -rn reject_workspace_dependencies crates/xtask`
零结果。

---

## 阶段 E — 封装与模块文档（可选档，做到这停）

### E1. 内部状态字段收私有

只处理"外部写入会破坏行为"的字段，不做全面封装运动：

- `DemoParticleEmitter2d::emission_accumulator` → 私有 +
  系统内部经方法推进（`fn accumulate(&mut self, amount: f32) -> usize`
  返回应生成数，顺带把 while 循环的逻辑收进类型，正好可被阶段 C 纯逻辑测试）。
- `DemoFrameAnimation2d::elapsed_seconds` → 私有 +
  `fn tick(&mut self, delta: f32) -> bool`（返回是否该换帧）。

配置语义的字段（速率、寿命、颜色尺寸）保持 `pub`——它们就是给人调的。

### E2. 模块级 `//!` 文档

- 先人工补齐 demo 相关文件的一行 `//!`（27 个文件 × 一句话，约半小时）。
- 然后再决定是否上规则：xtask 新增"`src/**/*.rs` 必须以 `//!` 开头"
  对全 workspace 是大动作，**先只对 demo 文件人工补齐，规则暂不加**，
  观察后续 AI 写新文件时是否会模仿（现在协议骨架和 demo 都有了文档示范，
  模仿概率比 05 时期高）。一个月后还是不写，再上规则。

**验收**：全套命令绿；`grep -L '^//!' crates/render_2d/src/**/demo_*.rs`
零结果（其余 demo 目录同理抽查）。

---

## 完成定义

- 摩擦表（05 §16）每行有裁决，无未决条目。
- 上面五个阶段的验收全部通过。
- `cargo run` 手工过一遍 05 §2 的玩家视角描述 + 阶段 B4 的组合路径。

到这里就是 9 分。之后停下来——下一份值得写的文档应该是真实游戏的第一个特性，
而不是 07-继续打磨。
