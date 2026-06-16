# xtask 规则功能审计

本文档只记录当前 `crates/xtask/src/rules/` 已经在检查什么，方便人工判断哪些规则应该保留、修改或删除。

标记说明：

- `核心边界`：建议保留，属于 crate 职责、依赖方向、运行时边界。
- `结构约定`：有价值，但容易随目录设计变化，需要人工确认。
- `临时快照`：强依赖当前 demo、文件名、函数名或具体字符串，长期风险高。
- `疑似过细`：更像 AI 为了防错写出的风格检查，可能应该删掉或改成文档建议。

## 全局规则

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| `check_workspace_readability` | 要求每个 crate 的 `src/lib.rs` 以 `//!` crate 文档开头；Rust 文件不得超过 400 行。 | `疑似过细`，文件长度限制可能有用，但 400 行是硬编码风格规则。 |
| `require_animated_2d_frame_manifests` | `assets/2d/animated/**/*.png` 必须有同名 `.frames.ron`。 | `核心边界`，和 frame animation 资源协议有关。 |

## 通用检查能力

| 模块 | 当前作用 | 判断 |
| --- | --- | --- |
| `base/dependencies.rs` | 检查 workspace 依赖、禁止依赖、禁止 manifest 字符串、禁止其它 crate 直接依赖某些后端。 | `核心边界`，但字符串匹配要避免检查具体版本。 |
| `base/derives.rs` | 禁止某些目录定义 `Component`、`Bundle`、`Resource`、`Event`、`Message`。 | `核心边界`，适合表达 crate 数据归属。 |
| `base/source.rs` | 禁止直接输入、网络传输、Bevy World 访问、世界 mutation 类型等。 | `核心边界`，但部分是字符串匹配，需要谨慎。 |
| `base/paths.rs` | 要求/拒绝路径、限制目录文件、限制子目录。 | `结构约定`，容易变成脆弱路径白名单。 |
| `base/functions.rs` | 禁止 free function 返回 `impl Bundle` 或 `Node`。 | `疑似过细`，只适合少数 UI/render 目录。 |
| `base/render_api.rs` | 同一 render 文件只允许一个 public `Bundle` 产品入口。 | `结构约定`，符合“产品入口清楚”，但可能误伤复杂产品。 |

## app

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| crate 必须存在并有 `src/lib.rs`。 | 保证 runnable app crate 存在。 | `结构约定` |
| 禁止依赖 `ecs/audio/intent/physics/prefab/render_2d/render_3d/network/msrt-udp`。 | app 只能组装 gameplay/external adapter，不碰底层。 | `核心边界` |
| 禁止出现 `EcsPlugin/IntentPlugin/PhysicsPlugin/PrefabPlugin/Render2dPlugin/Render3dPlugin`。 | 防止 app 直接注册底层插件。 | `核心边界` |

## audio

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `source`、`spatial` 目录。 | 声音源和空间音频分组。 | `结构约定` |
| 禁止依赖 gameplay、prefab、render、physics、network 等。 | audio 保持基础播放层。 | `核心边界` |
| 禁止直接读取输入。 | audio 不处理外设。 | `核心边界` |

## catalog

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `crates/catalog/src/demo.rs`。 | demo 资源绑定集中在 catalog。 | `结构约定` |
| 禁止依赖 app/dev_preview/external_runtime/gameplay/intent/network/peripherals/physics。 | catalog 不参与运行时和玩法。 | `核心边界` |
| 禁止定义 ECS/Bevy 数据类型和访问 World。 | catalog 只绑定资源到 prefab 构造。 | `核心边界` |
| `demo.rs` 必须包含 `AssetServer`、`DemoPlayerPrefab`、`DemoGroundPrefab`。 | 确保当前 demo catalog 有指定内容。 | `临时快照` |
| gameplay/dev_preview 不得直接写指定 demo 资源路径。 | 强制通过 catalog 加载 demo 资源。 | `结构约定`，但路径清单是 `临时快照`。 |

## dev_preview

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `src/main.rs`、`src/previews/mod.rs`。 | preview 入口和场景调度分离。 | `结构约定` |
| 必须存在 `src/previews/demo_menu.rs`。 | 保留当前 UI demo preview。 | `临时快照` |
| 其它 crate 不得依赖 `dev_preview.workspace = true`。 | dev_preview 只能是顶层开发工具。 | `核心边界` |
| dev_preview 不得定义 ECS 数据类型。 | preview 只组装生产 crate。 | `核心边界` |
| `main.rs` 不得同时出现 `App::new` 和 `.run`。 | main 只负责选择 preview。 | `结构约定` |
| `previews/mod.rs` 不得出现 `App::new/.add_plugins/.add_systems/Commands`。 | mod 只分发，不写具体 preview。 | `结构约定` |
| `src` 根目录除 `main.rs` 外不允许文件。 | 强制 preview 都放进 `previews`。 | `疑似过细` |

## ecs

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 拒绝旧路径 `crates/components`、`crates/system`。 | 防止回到旧结构。 | `结构约定` |
| 要求 `components/resources/events/systems` 目录。 | ECS 数据和行为分区。 | `核心边界` |
| `components` 下要求统一 README，拒绝其它 README。 | 文档集中。 | `疑似过细` |
| `components/events` 下禁止 `_system` 函数。 | 系统放 `systems`。 | `核心边界` |
| `resources` 禁止 `Component/Bundle/Event`。 | Resource 数据归属清楚。 | `核心边界` |
| `events` 禁止 `Component/Bundle/Resource/Event`，要求事件类型 derive `Message`。 | Bevy event/message 统一使用 `Message`。 | `核心边界` |
| `systems` 禁止定义 ECS 数据 derive。 | 行为和数据分离。 | `核心边界` |

## error

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| error crate 必须存在并有 `src/lib.rs`。 | 共享错误 crate。 | `结构约定` |
| 禁止 Bevy 依赖、`use bevy`、Bevy derive、Plugin。 | error 保持纯 Rust 错误层。 | `核心边界` |
| 所有其它 crate 必须依赖 `error.workspace = true`。 | 强制统一错误类型。 | `疑似过细`，并非所有 crate 都一定需要 error。 |
| 禁止其它 crate 定义 `Result` alias 或直接使用 `std/core::result::Result`。 | 强制使用 `error::Result`。 | `疑似过细`，可能误伤标准库/第三方边界。 |

## external_runtime

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `input/ai`、`input/network`、`runtime`、`manager`、`bridge` 目录。 | 外部 runtime 分区。 | `结构约定` |
| 拒绝 `input/local/device/peripherals` 等路径。 | 本地外设不放 external_runtime。 | `核心边界` |
| 禁止依赖 `ecs/audio/physics/render_2d/render_3d`。 | external_runtime 不碰底层世界表现。 | `核心边界` |
| 禁止 `InputPlugin/ExternalRuntimePlugin/impl Plugin for`。 | external_runtime 不做 Bevy plugin。 | `结构约定`，但可能需要重新评估。 |
| 禁止直接输入、runtime 下禁止 Bevy World、禁止世界 mutation。 | 通过 manager/bridge 通信。 | `核心边界` |
| 必须依赖 `network`、`toolcraft-config`。 | external_runtime 可选启动网络并读配置。 | `结构约定` |
| `config.rs` 必须包含 `toolcraft_config::load_settings/network/enabled`。 | 固定当前配置实现。 | `临时快照` |
| `runtime/task.rs` 必须包含 `Option<NetworkSourceConfig>`、`NetworkSource::connect`。 | 固定当前网络启动实现。 | `临时快照` |
| `input/network/source.rs` 必须包含 `NetworkClient/TocRouter/network::request`。 | 固定当前网络接入实现。 | `临时快照` |
| manager 用户 API 文件不得出现 `GameplayEntityId`。 | 用户 API 使用 runtime id，不暴露 gameplay id。 | `核心边界` |

## gameplay

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `api/lifecycle/schedule/interaction` 目录。 | gameplay 分层。 | `结构约定` |
| 禁止依赖 `ecs/audio/external_runtime/physics/render_3d/network/msrt-udp`。 | gameplay 通过 prefab/intent/API 接底层，不直接碰 runtime/network。 | `核心边界` |
| 禁止定义 `Component/Bundle/Resource/Event`。 | gameplay 不做 ECS 数据定义。 | `核心边界` |
| `Message` 只能在 `src/api` 下定义。 | gameplay public API 集中。 | `核心边界` |
| 禁止直接输入。 | 输入先由 peripherals/interaction 转语义消息。 | `核心边界` |
| 禁止 `GameplayManager/ExternalRuntimeManager`。 | manager 属于 external_runtime。 | `核心边界` |
| `interaction/mod.rs` 不得写 `MessageReader/match/info!` 等逻辑。 | mod 只导出，具体逻辑放文件。 | `结构约定` |
| 必须有 `interaction/ui`，且 interaction 根只允许 `mod.rs`。 | 交互逻辑按 domain 分类。 | `结构约定` |
| interaction 子目录只允许 `ui`。 | 新交互 domain 必须先改规则。 | `疑似过细` |
| 必须有 `interaction/ui/demo_menu.rs`，且包含指定符号。 | 固定当前 demo menu 交互实现。 | `临时快照` |

## helper

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| helper crate 必须存在并有 `src/lib.rs`。 | 共享基础设施 crate。 | `结构约定` |
| 禁止依赖所有游戏层、表现层、runtime/network。 | helper 不含游戏逻辑。 | `核心边界` |

## intent

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| intent crate 必须存在并有 `src/lib.rs`。 | 语义意图 API 边界。 | `结构约定` |
| 禁止依赖 `ecs/audio/physics/render_2d/render_3d/network/msrt-udp`。 | intent 不直接碰底层。 | `核心边界` |
| 禁止定义 ECS 数据 derive。 | intent 只写意图，不定义组件/资源。 | `核心边界` |
| 禁止直接输入和世界 mutation 类型。 | source handling 在 peripherals/external_runtime。 | `核心边界` |

## interaction

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 必须有 `src/message.rs`。 | 语义交互消息集中。 | `结构约定` |
| 禁止依赖 physics/prefab/render/external_runtime/audio。 | interaction 只做 Bevy interaction 到语义消息。 | `核心边界` |
| 禁止世界 mutation、网络传输。 | interaction 不改世界、不发网络。 | `核心边界` |
| `message.rs` 必须包含 UI navigation 相关类型和变体。 | 固定当前 UI 导航输入模型。 | `临时快照` |

## navigation

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `agent/target/path/query/systems` 目录。 | navigation 分区。 | `结构约定` |
| 禁止依赖 external_runtime/intent/gameplay/prefab/physics/render/network。 | navigation 保持世界导航基础层。 | `核心边界` |
| 禁止直接输入。 | 导航目标来自上层。 | `核心边界` |
| 禁止 Sprite/Camera/Text/Node/ImageNode。 | 可视化不放 navigation。 | `核心边界` |
| 禁止 import 上层 crate。 | 防止反向依赖。 | `核心边界` |

## network

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `connection/handler/protocol/request/router` 目录。 | 网络客户端分区。 | `结构约定` |
| 必须依赖 `msrt-udp/cmdproto/fnroute/prost/tokio`。 | 固定当前网络技术栈。 | `结构约定`，其中具体依赖名是项目选择，版本不应限制。 |
| 根 `Cargo.toml` 不得有 `path = "../cmdproto"` / `path = "../fnroute"`。 | 模板 CI-safe，不依赖本地父目录。 | `核心边界` |
| 禁止依赖 Bevy/gameplay/prefab/render/physics 等。 | network 是前端连接/协议层，不碰 Bevy world。 | `核心边界` |
| 禁止定义 Bevy ECS 数据和 gameplay message。 | network 不属于 Bevy ECS。 | `核心边界` |
| 禁止 Bevy World、直接输入。 | network 不读本地外设，不接 Bevy world。 | `核心边界` |
| 禁止 RuntimeRequestMessage/RuntimeUserId/RuntimeObjectId。 | id 映射属于 external_runtime。 | `核心边界` |
| `connection/client.rs` 必须包含 `msrt_udp/UdpClient/Reconnecting`。 | 固定当前 msrt-udp 包装实现。 | `临时快照` |
| 禁止 `UdpServer/NetworkSessionId/NetworkPeerId/src/session`。 | frontend network 不做服务端 peer/session。 | `核心边界` |
| `router/toc.rs`、`request/tos.rs`、`handler/*.rs` 必须包含具体符号。 | 固定当前 cmdproto/fnroute demo 实现。 | `临时快照` |
| handler 目录禁止 `route_toc/TocRouter/HashMap/Input<T>`。 | handler 只写具体处理函数，注册在 router。 | `结构约定` |

## peripherals

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 `keyboard/mouse/gamepad` 目录。 | 本地外设按设备分类。 | `结构约定` |
| 禁止依赖 ecs/physics/prefab/render/external_runtime/audio。 | peripherals 只把本地输入转语义消息。 | `核心边界` |
| 必须依赖 interaction。 | 本地输入通过 interaction message 发布。 | `核心边界` |
| 禁止定义 ECS 数据、世界 mutation、网络传输。 | 不改世界、不发网络。 | `核心边界` |
| 拒绝 `crates/peripherals/src/ui`。 | UI interaction 属于 interaction，不属于 peripherals。 | `结构约定` |

## physics

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| physics crate 和 `backend/rapier` 必须存在。 | Rapier 是当前唯一后端。 | `核心边界` |
| 要求大量具体 facade/backend 文件存在。 | 锁定当前 physics 目录设计。 | `临时快照`，路径白名单很重。 |
| 拒绝旧路径和 avian/rapier2d/rapier3d 旧布局。 | 防止结构回退。 | `结构约定` |
| 其它 crate 不得依赖物理后端 crate，physics 除外。 | 后端隔离，只通过 physics facade。 | `核心边界` |
| physics manifest 禁止 `[features]`、avian。 | 不做多后端 feature 切换。 | `结构约定` |
| physics `src/lib.rs` 不得 re-export 后端。 | 对外只暴露项目 facade。 | `核心边界` |
| physics 不得包含 Hitbox/Hurtbox/AttackRange/SkillRange。 | 玩法判定不放物理层。 | `核心边界` |

## prefab

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| prefab crate 必须有协议 anchor 和 `src/lib.rs`。 | prefab 是对象模板边界。 | `结构约定` |
| 禁止依赖 external_runtime/intent/gameplay/network/msrt-udp。 | prefab 不决定时机，不碰外部源。 | `核心边界` |
| 禁止直接输入。 | prefab 只组合对象数据。 | `核心边界` |
| 禁止 prefab 内调用另一个 prefab 的 `.spawn(commands)`。 | 多 prefab 编排由 gameplay 决定。 | `核心边界` |
| 禁止 `.spawn((`、`.insert((`。 | 不允许散装 tuple 生成/插入。 | `核心边界` |
| 禁止所有 `.insert(`。 | prefab 根对象必须完整 bundle。 | `核心边界`，但可能误伤运行时附加窄组件。 |
| `prefab/src/ui` 禁止 TextFont/TextColor/BackgroundColor/Color/px 等表现细节。 | UI 视觉在 render_2d，prefab 只组合。 | `核心边界` |
| 拒绝 `ui/camera.rs`、`ui/menu.rs`。 | UI camera 不放 prefab，demo 文件要显式命名。 | `结构约定` |
| public 字段不得是 `Entity`。 | prefab public API 不暴露 raw Entity。 | `核心边界` |
| 禁止 `UiCameraTarget`。 | UI camera 绑定由 gameplay spawn 后插入。 | `结构约定`，需要和最新 UI 设计复核。 |
| 禁止硬编码 asset path 后缀和目录。 | prefab 不加载/绑定具体资源路径。 | `核心边界` |

## render_2d 总体

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 必须有 `primitives/capabilities/products` 目录树和大量子目录。 | 锁定当前 render_2d 分组。 | `结构约定`，目录清单很重。 |
| 拒绝旧 flat modules 和旧 facade 目录。 | 防止结构回退。 | `结构约定` |
| 禁止依赖 ecs/external_runtime/audio/intent/prefab/physics/render_3d/network/msrt-udp。 | render_2d 是表现层，不碰 gameplay/runtime。 | `核心边界` |
| 禁止直接输入和世界规则术语。 | render 不驱动 gameplay。 | `核心边界` |
| 禁止硬编码 sprite sheet 切片 API。 | frame layout 来自 `.frames.ron`。 | `核心边界` |
| 禁止 `UiCameraTarget`。 | runtime UI camera 绑定不放 render_2d。 | `结构约定` |
| UI products 禁止 free function 返回 `impl Bundle` 或 `Node`。 | UI 表现用命名 bundle。 | `结构约定` |
| 禁止一个 UI root 同时组合多个 Node。 | 防止 Bevy UI 同 entity 多 Node。 | `核心边界` |
| 每个 render 文件只允许一个 public Bundle 产品入口。 | 产品 API 清楚。 | `结构约定`，可能误伤复杂产品。 |
| products 禁止 `pub fn into_bundle(self) -> impl Bundle`。 | prefab 消费命名 bundle。 | `核心边界` |
| 禁止 `common.rs/misc.rs/utils.rs`。 | 文件名必须语义化。 | `疑似过细` |

## render_2d primitives/capabilities 子规则

| 模块 | 当前作用 | 判断 |
| --- | --- | --- |
| `atlases` | 只能有 `mod.rs/plugin.rs`，不允许子目录；必须包含 `AtlasSprite2d` 相关具体 API；禁止加载资源和 Timer。 | `临时快照`，API 字符串检查过细。 |
| `camera` | root 只能有 `mod.rs/base.rs/plugin.rs`，presets 只能有 `fixed/follow/ui`。 | `结构约定`，但过早限制扩展。 |
| `frame_animation` | 不允许子目录和 `base/content/demo/example` 文件；禁止 demo public API、资源加载、硬编码切片。 | 边界部分是 `核心边界`，文件名限制是 `结构约定`。 |
| `skeletal_animation` | 每个产品必须是目录，必须有 `mod/entry/systems/tests` 和 `rig` 下固定文件。 | `结构约定`，目录清单很重。 |
| `tilemap` | 只能有 `mod/chunk/plugin`，必须包含具体 TilemapChunk API 字符串，禁止 demo 常量。 | `临时快照`，API 字符串检查过细。 |
| `images` | 只能有 `mod.rs`，必须包含 `StaticImage2d` API 字符串，禁止资源加载和 systems。 | `临时快照` |
| `text` | 只能有 `mod/plugin`，必须包含 `WorldText2d` API 字符串，禁止资源加载和 UI Node。 | `临时快照` |

## render_3d

| 规则 | 当前作用 | 判断 |
| --- | --- | --- |
| 要求 animation/camera/characters/debug/effects/environment/items/lighting/materials/models/overlays/particles/props/scenes 目录。 | 3D 表现目录分组。 | `结构约定` |
| 禁止依赖 external_runtime/audio/intent/prefab/physics/render_2d。 | 3D 表现层边界。 | `核心边界` |
| 禁止直接输入和世界规则术语。 | render_3d 不驱动 gameplay。 | `核心边界` |

## 明显应该优先复核的规则

这些规则最容易造成“为了过 xtask 而扭曲代码”：

1. 固定文件内容字符串：`require_file_contains_all_terms` 大量检查具体函数名、类型名、实现名。
2. 固定目录白名单：physics、render_2d primitives、skeletal、camera、dev_preview 当前都很重。
3. 固定 demo 文件：`demo_menu.rs`、catalog demo 路径、network demo protobuf handler。
4. 全局风格规则：400 行文件限制、禁止 `common.rs/utils.rs`、强制所有 crate 依赖 `error`、禁止标准 `Result`。
5. prefab 禁止所有 `.insert(`：方向对，但可能需要区分“spawn 构造期补组件”和“合法窄桥接”。
6. render 文件只允许一个 public bundle：方向对，但复杂 product 可能需要多个 public 类型，例如 product + config。

## 建议清理方向

第一阶段只保留：

- crate 依赖方向。
- 是否允许 Bevy World / Commands / Query / input / network transport。
- ECS 数据归属：Component、Bundle、Resource、Message 放在哪里。
- prefab 不散装 spawn/insert，不硬编码资源路径。
- render_2d 不依赖 ecs，不写 gameplay 规则，不硬编码 sprite sheet 切片。
- physics 后端隔离。
- network 不碰 Bevy、不做服务端 session。

第二阶段把以下规则删除或改成文档建议：

- 具体文件必须存在。
- 具体函数名/类型名必须出现在某文件。
- 具体 demo 文件、demo 资源路径。
- 具体目录只允许某些文件。
- 固定依赖版本。
- 容易误伤的风格规则。
