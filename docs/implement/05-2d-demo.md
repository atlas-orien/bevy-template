# 05 - 2D 游戏 Demo

这份文档描述如何在当前框架内实现一个完整的 2D demo：可控制的 player、
视差背景、tilemap、粒子、动画、物理感应区、音频、头顶覆盖层，
以及可选的 AI NPC。

注意：本文是实现路线说明，不是项目硬约束来源。项目硬约束只来自：

- `AI_PROTOCOL/*.md`
- `crates/xtask/src/rules/*.rs`

## 0. 目的与原则

这个 demo 同时承担三个角色：

1. **框架验证**：每条管线（输入 → 请求 → intent → ecs 规则 → 表现）都被真实代码走一遍，
   设计问题在 demo 里暴露，而不是在真实项目里暴露。
2. **AI 的"正确形状"参照**：AI 写代码时模仿就近例子的权重高于遵守禁令。
   demo 是它能找到的最近的例子，所以 demo 的每一行都必须放在协议规定的落点上。
3. **规则调整的触发器**：规则永远在写例子的时候调整。demo 撞上某条
   AI_PROTOCOL 或 xtask 规则时，不是绕过去，而是记录下来由人决定改规则还是改 demo
   （见文末"规则摩擦记录"）。

贯穿全文的硬性约定：

- **所有 demo 内容用 `Demo` / `demo_` 前缀命名**（类型 `DemoPlayerPrefab`、文件
  `demo_player.rs`、action id `"ui.demo.start"`）。这是已有惯例
  （`prefab/src/ui/demo_menu.rs`），延续它，使整个 demo 可以用 `grep -ri demo` 机械删除。
- **demo 不引入新依赖**。bevy 自带 sprite / tilemap chunk / audio，粒子用 sprite 实体实现，
  不引 `bevy_hanabi`、`bevy_ecs_tilemap` 等三方库。模板的价值是展示分层，不是展示库。
- **每个里程碑独立可验收**，做完一个合一个，不要攒一个大改动。

## 1. 现状盘点（实现前必读）

以下是 demo 要接入的既有事实，写代码前先核对它们还成立：

| 事实 | 位置 |
|---|---|
| `AppState::Loading` 进入后立即切到 `Playing` | `crates/gameplay/src/state/mod.rs` 的 `enter_loading` |
| `OnEnter(Playing)` 执行 spawn plan + 生成 UI camera 和 `DemoMenuPrefab` | `crates/gameplay/src/spawning/initial.rs` |
| 默认 spawn plan 为空 | 同上 `default_gameplay_spawn_plan()` |
| demo 菜单四个按钮的 action 目前只打 log | `crates/gameplay/src/interaction/ui/demo_menu.rs` 的 `run_demo_menu_action` |
| Update 调度链：ReceiveRuntimeRequests → ConsumeRuntimeRequests → SyncRuntimeUpdates → GameplayRules | `crates/gameplay/src/schedule/sets.rs` |
| 键盘绑定已有 `Gameplay` 上下文的 WASD/Space/Escape，但**没有任何系统消费它们** | `crates/peripherals/src/keyboard/bindings/gameplay.rs` |
| peripherals 目前只发 UI 导航，且硬编码 `LocalInputContext::UiNavigation` | `crates/peripherals/src/keyboard/mod.rs` |
| 移动链路已端到端打通：`MovementIntent` → `movement_system` 改 `Transform` | `crates/ecs/src/systems/movement/mod.rs` |
| `intent::movement::set_movement_intent(entity, target, query)` 可用 | `crates/intent/src/movement/mod.rs` |
| `prefab::identity::find_gameplay_entity(id)` 可用 | `crates/prefab/src/identity/mod.rs` |
| render_2d 每个分类目录有 `example.rs` 占位（`ExampleTilemapLayer2dBundle`、`ExampleBackground2dBundle`、`ExampleParticleEmitter2dBundle` 等），协议允许替换或删除 | `crates/render_2d/src/*/example.rs` |
| 资产目录已建好空骨架 | `assets/2d/static/{tilemaps,backgrounds,characters,...}`、`assets/audio`、`assets/levels` |
| tilemap 用 bevy 内置 `TilemapChunk` / `TilemapChunkTileData` | `crates/render_2d/src/tilemap/example.rs` |

另外两条影响实现方式的协议边界，写 demo 时最容易踩：

- `gameplay` **禁止依赖** `ecs`、`audio`、`physics`、`external_runtime`、`render_3d`
  （`crates/xtask/src/rules/crates/gameplay.rs`）。gameplay 要摸 ECS 数据时，
  一律走 `prefab` 暴露的窄 facade（参照 `prefab::identity` 和 `prefab::intent` 的现成做法）。
- `render_2d` 可以读 `ecs` 组件决定显示，但不依赖 `prefab` / `physics` / `intent`，
  不读输入，不执行世界规则。

## 2. Demo 范围总览

完成后 `cargo run` 应该看到：

> 启动进入主菜单 → 点 Start → 出现 tile 地面、两层视差背景、一个可以用 WASD
> 控制的角色（带行走动画和朝向翻转、脚下扬尘粒子、头顶血条）、几个静物障碍、
> 一个踩上去会触发音效和粒子爆发的感应区；Escape 暂停/恢复；
> （可选档）一个自己游荡的 AI NPC。

功能 × crate 覆盖矩阵（demo 做完，打 √ 的格子都有真实代码）：

| 功能 | peripherals | interaction | gameplay | intent | prefab | ecs | render_2d | physics | audio | navigation | external_runtime |
|---|---|---|---|---|---|---|---|---|---|---|---|
| M1 状态流与菜单 | √ | √ | √ | | √ | | √ | | | | |
| M2 相机与背景 | | | √ | | √ | | √ | | | | |
| M3 tilemap | | | √ | | √ | | √ | | | | |
| M4 可控 player | √ | | √ | √ | √ | √ | √ | | | | |
| M5 相机跟随 | | | | | √ | √ | √ | | | | |
| M6 帧动画 | | | | | √ | √ | √ | | | | |
| M7 粒子 | | | | | √ | | √ | | | | |
| M8 感应区 | | | √ | | √ | √ | | √ | | | |
| M9 音频 | | | | | √ | √ | | | √ | | |
| M10 覆盖层 | | | | | √ | √ | √ | | | | |
| M11 AI NPC（可选） | | | √ | √ | √ | √ | √ | | | √ | √ |

里程碑依赖关系：M1 → M2 → M3 可顺序也可并行（互不依赖代码）；
M4 依赖 M1（需要 Playing 状态）；M5–M10 都依赖 M4（需要 player 存在）；
M11 依赖 M3（需要可走的地面）。

## 3. M0 — 资产准备

写代码前先把资产放到位，避免实现到一半临时找图。

**最低需求清单：**

| 资产 | 落点 | 规格建议 |
|---|---|---|
| tileset | `assets/2d/static/tilemaps/demo_tileset.png` | 16×16 或 32×32 每 tile，至少 4 种 tile（草、土、石、装饰） |
| 角色 sprite sheet | `assets/2d/animated/characters/demo_player.png` | 行 = 动画（idle / walk），列 = 帧，每帧尺寸固定 |
| 背景远层 | `assets/2d/static/backgrounds/demo_far.png` | 可横向平铺 |
| 背景近层 | `assets/2d/static/backgrounds/demo_near.png` | 可横向平铺，带透明 |
| 粒子贴图 | `assets/2d/static/particles/demo_dust.png` | 8×8 左右的小白点，运行时染色 |
| 静物 | `assets/2d/static/props/demo_rock.png` | 单张 |
| 脚步声 | `assets/audio/demo_footstep.ogg` | < 0.5s |
| 感应区音效 | `assets/audio/demo_pickup.ogg` | < 1s |
| BGM | `assets/audio/demo_bgm.ogg` | 可循环，体积控制在 1MB 内 |

**来源**：推荐 Kenney（kenney.nl，CC0）的 pixel 包，一次下载基本全凑齐。
**降级方案**：除角色 sheet 和 tileset 外，其余都可以先用纯色占位——
`Sprite { color, custom_size }` 不需要贴图（`ExampleBackground2dBundle::new` 就是这么写的），
音频可以最后补。不要让找资产阻塞 M1–M3。

**验收**：文件就位，`cargo run` 无新增 asset 加载错误（此时还没人引用它们，主要确认路径和格式约定）。

## 4. M1 — 状态流与菜单接线

**目标**：把现有"启动即 Playing + 菜单悬空打 log"改成真实游戏流程：
`Loading → MainMenu`（显示菜单）→ Start → `Playing`（进世界）；
Playing 中 Escape ↔ `Paused`；菜单 Quit 退出进程。

这是 demo 第一个里程碑，因为它不引入任何新对象，
只把已有的状态机、菜单、interaction 链路接成真的，并为后续所有 spawn 提供正确时机。

**改动落点：**

1. `crates/gameplay/src/state/mod.rs`：`enter_loading` 改为切到 `AppState::MainMenu`
   （模板没有真实加载任务，保留 Loading 状态本身，未来资产预载有落点）。
2. `crates/gameplay/src/schedule/enter.rs` / `exit.rs`：
   - `OnEnter(MainMenu)`：生成 UI camera + `DemoMenuPrefab`（把现在
     `spawn_initial_gameplay_plan_system` 里的 UI 部分挪过来）。
   - `OnExit(MainMenu)`：despawn 菜单实体树。需要一个 demo 菜单根 marker 查询；
     marker 已有（`render_2d::ui::DemoMenuRootBundle` 里的 root marker），
     清理系统写在 `crates/gameplay/src/cleanup/`（该目录就是为此预留的）。
     UI camera 不清理，全程复用。
   - `OnEnter(Playing)`：保留 `spawn_initial_gameplay_plan_system`，
     但删去其中的菜单生成（后续里程碑会往 plan 里加世界对象）。
3. `crates/gameplay/src/interaction/ui/demo_menu.rs` 的 `run_demo_menu_action`：
   - 现在的自由函数拿不到状态资源，需要把签名升级为可以访问
     `ResMut<NextState<AppState>>` 和 `MessageWriter<AppExit>`（通过上层系统传入，
     或把 match 直接合并进两个 handler 系统）。
   - `DEMO_START_ACTION` → `next_state.set(AppState::Playing)`。
   - `DEMO_QUIT_ACTION` → 写 `AppExit`。
   - `DEMO_OPTIONS_ACTION` / `DEMO_BACK_ACTION` → 保留 log（demo 不做设置页）。
4. 暂停：`peripherals` 的 Gameplay 绑定里已有 `Escape → Pause`，但暂停切换属于
   gameplay。在 M4 的本机输入链路做好之前，先在 gameplay 写一个临时系统直接读
   Escape 是不行的（gameplay 禁碰 `KeyCode`，xtask 会拦）。**所以暂停推迟到 M4
   一起做**，M1 只做 MainMenu/Playing/Quit 三条。

**设计决策（已定）**：菜单生成从 `OnEnter(Playing)` 移到 `OnEnter(MainMenu)` 是行为变更，
意味着模板"开箱即菜单"的演示效果不变，但状态语义从此正确。
`docs/implement/README.md` 里"Loading 会自动切到 Playing"的描述要同步更新。

**验收**：

```sh
cargo fmt --check && cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- check
cargo run   # 启动见菜单；Start 后菜单消失（世界还是空的）；Quit 退出进程
```

## 5. M2 — 世界相机与视差背景

**目标**：进入 Playing 后有一个 2D 世界相机和两层视差背景。

**改动落点：**

1. `crates/render_2d/src/camera/`：新增 `demo_camera.rs`（或把 `example.rs` 的
   marker 替换为真实实现）：`DemoWorldCamera2dBundle`，包含 `Camera2d`、
   world camera marker。注意 `Camera.order` 必须低于 `UiCamera`
   （RENDER_2D.md 的多 camera 规则）。
2. `crates/render_2d/src/background/`：把 `example.rs` 改成真实的
   `demo_background.rs`：`DemoBackgroundLayer2dBundle`（sprite + 视差速度分量），
   外加一个视差系统：读相机 `Transform`，按 `speed` 比例偏移各背景层的渲染位置。
   视差是纯表现逻辑，归 render_2d，系统注册进 `BackgroundPlugin`。
3. `crates/prefab/src/world_2d/`：新增 `demo_level/mod.rs`，先放
   `DemoBackgroundPrefab`：组合两层 `DemoBackgroundLayer2dBundle`
   （远层 speed 小、z 低；近层 speed 大、z 高一点，都在 tile 层之下）。
4. `crates/gameplay/src/spawning/initial.rs`：world camera 由 gameplay 直接 spawn
   （与 UI camera 同样的处理方式——camera 不是 prefab，PREFAB.md 明确 ui prefab
   不放 camera，世界相机同理）；背景通过
   `default_gameplay_spawn_plan().with(DemoBackgroundPrefab)` 进 plan。

**设计决策（已定）**：背景一个 prefab 管两层（一次 spawn 多个实体，
`Prefab::spawn` 返回根实体，两层作为子实体挂在一个空根下），
比两个 prefab 各管一层好——删除和层级关系都更简单。

**验收**：`cargo run` → Start → 能看到两层颜色/贴图不同的背景；
相机还不会动（M5 才跟随），视差暂时看不出效果，属预期；全套检查命令通过。

## 6. M3 — Tilemap 地面层

**目标**：Playing 世界里有一块由 tileset 渲染的地面。

**改动落点：**

1. `crates/render_2d/src/tilemap/`：把 `example.rs` 替换为 `demo_tilemap.rs`：
   `DemoTilemapLayer2dBundle`，结构沿用 `ExampleTilemapLayer2dBundle`
   （`TilemapChunk` + `TilemapChunkTileData` + marker），构造函数签名保持
   `(chunk_size, tile_display_size, tileset_handle, tile_indices)`。
2. `crates/prefab/src/world_2d/demo_level/`：新增 `ground.rs`：
   `DemoGroundPrefab`，持有 tileset `Handle<Image>` 和关卡数据，
   `spawn` 时构造 tilemap bundle。
3. **关卡数据落点（设计决策，已定）**：v1 用 `demo_level/layout.rs` 里的常量数组
   （`const DEMO_GROUND: [[u16; W]; H]`，配几个 tile 索引常量加注释）。
   不做 RON/LDtk 加载器——加载器是独立特性，不属于 demo 范围；
   `assets/levels/` 留给未来 v2，本文不展开。
4. tileset 的 `Handle<Image>` 从哪来：prefab 的 `spawn(self, commands)` 拿不到
   `AssetServer`。两个方案：
   - **方案 A（推荐）**：prefab struct 持有 handle，由 gameplay 在 spawn plan
     构造时通过 `AssetServer` 加载后传入。`GameplaySpawnPlan` 的构造点
     （`default_gameplay_spawn_plan`）目前是无参函数，需要升级为接收
     `&AssetServer`（`spawn_initial_gameplay_plan_system` 里加 `Res<AssetServer>` 即可）。
   - 方案 B：prefab spawn 后由 render_2d 系统看到 marker 再补贴图。多一次状态同步，不推荐。
   方案 A 同时解决 M4/M6/M7 所有需要贴图的 prefab，一次把口子开对。

**验收**：`cargo run` → Start → 背景之上出现 tile 地面；
故意把一个 tile 索引改成无效值确认渲染不 panic（再改回来）；全套检查命令通过。

## 7. M4 — 可控制 Player（demo 的核心里程碑）

**目标**：WASD 控制角色移动，Escape 暂停。这条链路打通后，
demo 才真正验证了框架的立项设计：

```text
peripherals（KeyCode → LocalInputAction → 语义消息）
→ gameplay（消费语义消息，找到受控实体）
→ intent（写 MovementIntent）
→ ecs::movement_system（改 Transform）
→ render_2d（显示）
```

### 7.1 设计决策（实现前确认，本文给出推荐）

**决策 1：本机移动输入用什么消息进 gameplay？**

现有 `RuntimeRequestMessage::SetMovementIntent { id, target }` 需要
`GameplayEntityId`，但 peripherals 不应该知道"player 是哪个实体"——
它只知道"本机用户想往左走"。

推荐：在 `crates/gameplay/src/api/` 新增本机语义消息
`LocalUserInputMessage`（Bevy `Message`），变体先只要两个：
`Move(Vec2 /*单位方向，零向量表示停*/)` 和 `TogglePause`。
peripherals 写它（PERIPHERALS.md 明确允许 peripherals 依赖 gameplay 提交语义请求），
gameplay 消费它。不复用 `RuntimeRequestMessage`：那是 runtime-id 边界的消息，
本机输入没有 runtime id，混在一起会让两个边界互相渗透。

**决策 2：gameplay 怎么知道哪个实体受本机控制？**

推荐：marker 组件 `DemoPlayerControlled` 定义在
`crates/ecs/src/components/characters/`（核心玩法数据归 ecs），
prefab 在 player bundle 里组合它，并在 `crates/prefab/src/identity/`（或新开
`crates/prefab/src/control/`）暴露查询 facade：

```text
prefab::control::LocallyControlledQuery   （type alias，模式照抄 prefab::identity）
```

gameplay 用这个 facade 找实体，再调 `intent::movement::set_movement_intent`。
gameplay 全程不 import `ecs`（xtask 禁止依赖会拦）。

**决策 3：输入上下文（UiNavigation / Gameplay）谁切换？**

PERIPHERALS.md 规定 peripherals 不拥有流程状态。推荐：
`crates/gameplay/src/api/` 定义 Resource `ActiveLocalInputContext`（包一个
`peripherals` 不可见的枚举不行——它就是 `LocalInputContext`，该类型在 peripherals。
为避免循环依赖，**枚举留在 peripherals，Resource 定义在 gameplay**，
gameplay 依赖 peripherals 是不行的——peripherals 已依赖 gameplay）。

所以修正为：`LocalInputContext` 枚举**搬到 gameplay::api**（它本来就是
"当前游戏处于什么输入语境"的流程语义，放 gameplay 比放 peripherals 更符合
"peripherals 不拥有流程状态"），peripherals 从 gameplay import 它。
gameplay 在 `OnEnter(MainMenu/Paused)` 设 `UiNavigation`、`OnEnter(Playing)` 设
`Gameplay`。peripherals 的键盘系统读这个 Resource 决定激活哪组绑定，
替换现在硬编码的 `LocalInputContext::UiNavigation`。

这是 demo 对框架的第一处真实修正（占位设计在真实使用时暴露的问题），
属于预期收获，记入"规则摩擦记录"。

### 7.2 改动落点

1. `crates/ecs/src/components/characters/`：`DemoPlayerControlled` marker。
2. `crates/render_2d/src/characters/`：`example.rs` 替换为 `demo_player.rs`：
   `DemoPlayerSprite2dBundle`（`Sprite`（先用单帧或纯色）+ marker）。
   按 RENDER_2D.md 的渲染实体规则，sprite 放在玩法实体的**渲染子实体**上。
3. `crates/prefab/src/world_2d/characters/`（目前是空文件）：`demo_player.rs`：
   `DemoPlayerPrefab`，组合：
   - ecs：`GameplayEntityId`、`DemoPlayerControlled`、`MovementIntent`、`Speed`、
     `Facing`、`Health`（M10 用）、`Transform`（初始位置）
   - 渲染子实体：`DemoPlayerSprite2dBundle`
   - 进 spawn plan：`default_gameplay_spawn_plan().with(DemoPlayerPrefab::new(...))`
4. `crates/gameplay/src/api/`：`LocalUserInputMessage` + `ActiveLocalInputContext`
   + `LocalInputContext`（从 peripherals 搬来）。
5. `crates/peripherals/src/keyboard/`：新系统
   `emit_keyboard_gameplay_input_system`：读 `ActiveLocalInputContext`，为
   `Gameplay` 上下文收集 `collect_keyboard_actions`，把
   MoveUp/Down/Left/Right 合成单位方向向量写
   `LocalUserInputMessage::Move(dir)`——**注意每帧都要发**（包括零向量），
   否则松开按键后角色滑行不停；`Pause` 动作发 `TogglePause`。
6. `crates/gameplay/src/interaction/`（或新开 `gameplay/src/control/`，推荐后者，
   interaction 目录按协议只放 ui 类）：消费系统
   `apply_local_user_input_system`：
   - `Move(dir)` → 经 `prefab::control` facade 找受控实体 →
     `intent::movement::set_movement_intent(entity, Direction(dir) or None, ...)`
   - `TogglePause` → Playing↔Paused 互切
   - 注册进 `GameplayUpdateSet::GameplayRules`，并用 `run_if(in_state(Playing))`
     约束（Paused 时只响应 `TogglePause`，单独一个不受 run_if 限制的小系统）。
7. `crates/gameplay/src/schedule/update.rs`：注册上述系统；
   `ecs::systems::movement::movement_system` 当前没有被任何调度注册——
   检查 `EcsPlugin`（`prefab` 注册）是否已挂它，没有则在此处补上
   （挂进 `GameplayRules` set，`run_if(in_state(Playing))`，这样 Paused 自动冻结移动）。
8. `OnEnter(Paused)` / `OnExit(Paused)`：M1 留下的暂停接线在这里完成；
   暂停时可顺手把菜单 prefab 复用为暂停菜单（Back → 恢复，Quit → 退出），
   这正好让 `DEMO_BACK_ACTION` 有了真实用途。

**验收**：

```sh
# 全套检查命令（同 M1）
cargo run
# Start 后 WASD 移动角色，松键即停；A/D 切换朝向（贴图翻转 M6 才有，
# 先用 info! 打 Facing 验证）；Escape 暂停（角色冻结、出现菜单），再 Escape 恢复
```

链路验证技巧：在 `apply_local_user_input_system` 和 `movement_system` 各打一条
`debug!`，确认消息每帧到达、零向量正确落地成 `MovementTarget::None`。

## 8. M5 — 相机跟随

**目标**：世界相机平滑跟随 player。

**改动落点：**

1. `crates/render_2d/src/camera/`：`DemoCameraFollowTarget` marker（渲染专用组件，
   归 render_2d）+ `demo_camera_follow_system`：读带 marker 实体的 `Transform`，
   对相机 `Transform` 做 lerp（系数写成组件字段，默认 ~5.0/s）。
   注册进 `CameraPlugin`，挂 `PostUpdate`（在 movement 之后、渲染之前）。
2. `crates/prefab/src/world_2d/characters/demo_player.rs`：player bundle 加
   `DemoCameraFollowTarget`（prefab 组合 render_2d 组件，协议允许）。

此时 M2 的视差背景应该能看出层间速度差了。

**验收**：`cargo run` → 移动时相机跟随、背景层错速移动；
把 lerp 系数调成极大值确认无抖动回归。

## 9. M6 — 帧动画

**目标**：player 有 idle / walk 两个动画，移动时播 walk，朝向翻转贴图。

**改动落点：**

1. `crates/render_2d/src/atlases/`：`demo_player_atlas.rs`：
   `TextureAtlasLayout` 构造函数（按 M0 的 sheet 规格切格子）。
2. `crates/render_2d/src/animation/frame/`：`example.rs` 替换为
   `demo_frame_animation.rs`：
   - `DemoFrameAnimation` 组件：当前动画行、帧范围、帧时长、计时器。
   - `demo_frame_animation_system`：tick 计时器推进 atlas index。
   - `demo_player_animation_state_system`：读父实体（玩法实体）的
     `ecs::MovementIntent` 和 `ecs::Facing`（render_2d 读 ecs，协议允许），
     切 idle/walk 行、设 `Sprite.flip_x`。动画状态机不进 ecs——它是纯表现状态。
3. `crates/render_2d/src/characters/demo_player.rs`：sprite bundle 升级为带
   atlas 的 sheet sprite + `DemoFrameAnimation`。
4. prefab 侧把贴图 handle 传入的口子 M3 方案 A 已开好，照用。

**验收**：`cargo run` → 站立播 idle，移动播 walk，向左走贴图水平翻转；
动画推进在 Paused 状态下应停止（系统挂 `run_if(in_state(Playing))`，
这条需要 render_2d 能引用 `AppState`——**不行**，render_2d 不依赖 gameplay。
改为：动画系统读 `Time` 正常推进，但暂停时 `MovementIntent` 已是 None、
状态机自然回 idle，可接受；若要求完全冻结，由 gameplay 控制 `Time` 的
virtual pause，这是 Bevy 自带能力（`Time<Virtual>::pause`），由 gameplay 在
OnEnter/OnExit(Paused) 调用——推荐做这个，顺便让粒子、视差全部正确冻结）。

## 10. M7 — 粒子

**目标**：player 移动时脚下扬尘；感应区触发时粒子爆发（爆发在 M8 接线）。

**改动落点（全部在表现层）：**

1. `crates/render_2d/src/particles/`：`example.rs` 替换为 `demo_particles.rs`：
   - `DemoParticleEmitter2d` 组件：发射速率、粒子寿命、初速度范围、颜色、是否启用。
   - `DemoParticle2d` 组件：剩余寿命、速度。
   - `demo_particle_emission_system`：按速率 spawn 粒子实体
     （sprite + `DemoParticle2d`），位置取发射器全局位置。
   - `demo_particle_update_system`：积分位置、按寿命衰减 alpha、寿命尽 despawn。
   - `demo_particle_burst` 辅助函数：一次性 spawn N 个粒子（M8 用）。
   - 粒子实体是渲染专用实体，由 render_2d 自己 spawn/despawn，协议明确允许。
2. `crates/render_2d/src/characters/` 或 particles 内：
   `demo_player_dust_system`：读玩法实体 `MovementIntent`，移动时启用脚下
   emitter、静止时停发（emitter 作为 player 的另一个渲染子实体，由 prefab 组合）。
3. `crates/prefab/src/world_2d/characters/demo_player.rs`：渲染子实体加 emitter。

**性能护栏**：emitter 给粒子总数上限字段（默认 256），超限丢帧不丢稳定性；
这也是给 AI 看的"表现系统要带护栏"的形状。

**验收**：`cargo run` → 移动时脚下连续扬尘、停下即停；粒子自行消失，
`bevy` 的 entity 数不随时间无限增长（debug overlay 或 log 抽查）。

## 11. M8 — 静物与物理感应区

**目标**：物理层至少被真实使用一次：一个感应区，player 走进去触发事件 →
gameplay 收到 → 粒子爆发 + 音效（音效 M9 接）。

**设计决策（已定）**：v1 **不给 player 加刚体**。`movement_system` 是直接写
Transform 的运动模型，引入动力学刚体意味着重写移动；demo 的目的是每层都用到，
不是把移动改成物理驱动。感应区用 sensor + 点查询即可成立。
"player 改 kinematic character controller"留作 v2 决策点，记入摩擦表。

**改动落点：**

1. `crates/prefab/src/world_2d/demo_level/`：`props.rs`：`DemoRockPrefab`
   （静物：`Transform` + render_2d 的 props sprite bundle；
   `crates/render_2d/src/props/example.rs` 同步替换为 `demo_props.rs`）。
   静物不挡路（v1 没有碰撞响应），纯场景丰富度。
2. `crates/prefab/src/world_2d/demo_level/`：`sensor_zone.rs`：
   `DemoSensorZonePrefab`：组合 `physics` 的 sensor marker + collider shape
   （`crates/physics/src/sensor`、`collider/shape.rs` 已有基础类型）+
   `GameplayEntityId` + 一个 ecs marker `DemoSensorZone`
   （放 `ecs/components/world/`）。
3. player 这边需要一个能被 sensor 看到的 collider：给 player bundle 加
   physics collider + kinematic/static 配置中**最小的那种**（只参与相交检测）。
   具体用 `physics` facade 的哪个组合，实现时按 `crates/physics/src/` 现状选，
   原则：不引入力学积分，不让 rapier 接管 Transform。
4. 事件链：physics 的碰撞事件（`crates/physics/src/events/collision.rs`）→
   ecs 或 gameplay 怎么消费？gameplay 禁依赖 physics。两条路：
   - **推荐**：`crates/ecs/src/systems/interaction/`（已预留）写系统消费
     physics 碰撞事件——**ecs 也不能依赖 physics 的话**（查
     `crates/xtask/src/rules/crates/ecs.rs` 确认），则桥接系统放
     `prefab`（PREFAB.md 允许 prefab 依赖 physics 且明确"窄桥接系统"模式，
     audio 桥接就是先例）：prefab 写
     `demo_sensor_bridge_system`：physics 碰撞事件 + `DemoSensorZone` marker →
     发一个 ecs 事件（`ecs/events/` 新增 `DemoSensorTriggeredEvent`）。
   - gameplay 再消费这个 ecs 事件？gameplay 不能 import ecs——事件类型经
     `prefab` re-export（窄 facade 模式，和 `prefab::intent` 一致）。
5. `crates/gameplay/src/control/`（M4 开的目录）：消费 sensor 事件，
   调 render_2d 的 `demo_particle_burst`（gameplay 可依赖 render_2d，
   demo 菜单已有先例）并打 log；M9 接音效。

这一段是 demo 里跨层协作最长的链路，**正是规则摩擦最可能出现的地方**：
physics 事件 → prefab 桥接 → ecs 事件 → prefab re-export → gameplay 消费。
如果实现时发现某一跳被 xtask 拦住或者绕得太扭曲，停下来记录摩擦，
不要硬绕——这种"链路太长"的信号正是规则需要调整的证据。

**验收**：`cargo run` → 走进感应区：粒子爆发 + log 一次（离开再进入可重复触发，
中间不重复触发）；全套检查命令通过。

## 12. M9 — 音频

**目标**：进 Playing 播 BGM 循环；脚步声随移动播放；感应区触发音效。

**改动落点（按 PREFAB.md 的 Audio 边界，桥接全在 prefab）：**

1. player prefab 组合 `ecs::components::base::AudioClips`，登记
   `demo_footstep` 槽位；sensor zone prefab 登记 `demo_pickup`。
2. `crates/prefab/src/lifecycle/`（或新开 `prefab/src/audio_bridge/`）：
   - 脚步：桥接系统读 `MovementIntent` 移动状态 + 计步 timer →
     `audio::PlayAudioRequest`（已有类型，`crates/audio/src/request.rs`）。
   - 感应区：消费 M8 的 `DemoSensorTriggeredEvent` → `PlayAudioRequest`。
   - BGM：消费"进入 Playing"信号。gameplay 不能直接调 audio，prefab 也看不到
     `AppState`（prefab 不依赖 gameplay）。用 ecs 资源/事件中转：
     `ecs/resources/session.rs` 已有 session 资源概念，gameplay 经 prefab facade
     标记 session 开始，prefab 桥接系统看到后发 BGM 请求。
     若实现时发现这条太绕，备选：BGM 由 `app` 组装层直接处理
     （app 配 Bevy 外壳，BGM 作为外壳级行为勉强成立）——两个方案都不优雅，
     **这是已知的设计缺口，必须记入摩擦表**，候选规则调整：允许 gameplay
     依赖 audio 的 request 层（只发请求不碰后端）。
3. 播放执行端：确认 `audio::AudioFoundationPlugin` 已被 prefab 注册
   （PREFAB.md 写明由 prefab 注册），没有则补上。

**验收**：`cargo run` → BGM 循环；走动有脚步声、停下即停；
感应区触发一声；Paused 时脚步声停（BGM 可继续，写明即可）。

## 13. M10 — 头顶覆盖层

**目标**：player 头顶有血条（世界空间 overlay），让 `overlays` 分类不再是占位。

**改动落点：**

1. player prefab 已组合 `Health`（M4 埋的）。
2. `crates/render_2d/src/overlays/`：`example.rs` 替换为 `demo_health_bar.rs`：
   血条背景 + 填充两个 sprite 子实体；`demo_health_bar_system` 读父玩法实体的
   `ecs::Health`，按比例缩放填充条。
3. prefab 给 player 挂血条渲染子实体。
4. demo 里没有伤害来源，血条永远满格不直观——让 M8 感应区顺便扣 10 血
   （gameplay 消费 sensor 事件时经 prefab facade 改 Health；
   `prefab` 需要为此开一个窄的 health facade，模式同 `prefab::intent`）。
   血量到 0 → `AppState::GameOver`（状态机里这个状态一直闲置着，正好用上；
   GameOver 进入后显示菜单、Start 重开——重开走 `ClearSession` 已有请求）。

**验收**：`cargo run` → 头顶血条随感应区触发递减；归零进 GameOver；
重开后世界重新生成、血条满格。这一条同时验证了 `ClearSession` 清理是否干净
（重开后实体数不翻倍——用 log 抽查）。

## 14. M11（可选档）— AI NPC 与寻路

**目标**：打通最后两个没用到的 crate：`external_runtime`（AI 输入源）和
`navigation`（路径跟随）。一个 NPC 每隔几秒随机选一个点走过去。

**改动落点：**

1. `crates/prefab/src/world_2d/characters/`：`demo_npc.rs`：组合
   `GameplayEntityId` + `MovementIntent` + `Speed` + navigation 的
   agent/target 组件（`crates/navigation/src/agent`、`target/destination.rs` 已有）
   + 复用 player 的 sprite/动画 bundle（换个颜色 tint）。
2. NPC 进 spawn plan，spawn 时用 `RuntimeSpawnContext::for_object` 注册
   runtime object id（走 `RuntimeRequestMessage::spawn_prefab`，
   这样 external_runtime 的 manager 能看到它——`sync_gameplay_entities_system`
   已有同步链路）。
3. `crates/external_runtime/src/input/ai/`：实现一个最小源：
   `poll_external_sources` 周期里，每 N 秒为已注册的 demo NPC object id 调
   `set_object_movement_intent(object_id, MovementTarget::Position(随机点))`
   （manager 自由函数已存在）。随机点范围用常量框在 tilemap 内。
4. navigation 的 follow/refresh 系统（`navigation/src/systems/`）确认被注册
   （`NavigationPlugin`），直线 query（`straight_line.rs`）对 demo 够用。

**验收**：`cargo run` → NPC 自主游荡；关掉 external_runtime 的 AI 源
（注释掉 poll 注册）NPC 立即静止——证明驱动确实来自 App 外部。

## 15. Demo 的删除路径

demo 同时也要示范"怎么干净地退出模板"。完成后在本文件追加一节实测过的删除清单，
基本形态：

```sh
grep -rli demo crates/ assets/ | sort   # 列出全部 demo 文件
# 预期：prefab/world_2d/demo_level、prefab/world_2d/characters/demo_*、
# prefab/ui/demo_menu.rs、render_2d 各分类的 demo_*.rs、
# gameplay/interaction/ui/demo_menu.rs、gameplay/control 的 demo 部分、
# ecs 里的 Demo* 组件与事件、assets 的 demo_* 资产
```

删除后 `cargo check --workspace` 必须只在"引用 demo 的注册点"
（spawn plan、schedule 注册、plugin 组装）报错，顺着编译错误清理注册点即可回到空模板。
如果删除时发现 demo 类型渗进了非 demo 文件的核心逻辑里，那是实现犯规，回头修。

## 16. 规则摩擦记录

实现过程中每次撞上 AI_PROTOCOL / xtask 规则，按下表追加记录（直接编辑本节）。
不要默默绕过，也不要顺手改规则——规则变更由人看完摩擦记录后统一决定
（与 `04-ai-constraint-hardening.md` 阶段 A 的条款一致）。

| # | 里程碑 | 撞上的规则 | 现象 | 临时处理 | 建议 |
|---|---|---|---|---|---|
| 0 | M4 | PERIPHERALS.md：peripherals 不拥有流程状态 | `LocalInputContext` 放 peripherals 导致上下文切换无主 | 枚举搬到 gameplay::api，peripherals 读取 gameplay 的 `ActiveLocalInputContext` | 协议同步更新代码落点 |
| 1 | M4 | GAMEPLAY.md：状态退出调度承载 session 清理 | `Paused` 是独立 `AppState` 时，Playing → Paused 会触发 `OnExit(Playing)`，若在这里清理 gameplay session 会把暂停中的世界删掉 | 暂时移除 `OnExit(Playing)` 的 session 清理；暂停只生成/清理 demo 菜单，恢复时跳过重复 spawn | 明确 session lifecycle 与 flow state 的边界；候选方案是新增独立 pause substate，或改为显式 `ClearSession` 请求清理 session |

**已预见、实现时需要确认的摩擦点：**

1. PREFAB.md"模板本身不携带默认内容 prefab 或默认内容资源" vs demo 就是内容。
   候选解决：协议改为"非 `demo_` 前缀的默认内容不允许；demo 内容必须可按
   §15 清单整体删除"。
2. M9 的 BGM 触发链路过长，候选放宽：gameplay 可依赖 audio 的 request 类型。
3. M8 的 physics 事件 → gameplay 链路要经过两次 prefab 中转，实现后评估是否
   值得给 prefab 加一个统一的"事件 re-export" facade 约定。
4. `ecs::systems::movement::movement_system` 的注册归属（EcsPlugin 还是
   gameplay schedule）目前协议没写死，实现时确定后补进 ECS.md / GAMEPLAY.md。

## 17. 通用验收

每个里程碑收尾必跑：

```sh
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- check
cargo run        # 按各里程碑的运行观察点人工确认
```

全部里程碑完成的总验收，就是 §2 那段玩家视角描述逐句成立。
