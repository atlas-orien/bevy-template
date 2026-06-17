此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# PREFAB

这个文件是 `crates/prefab` 的 AI 规则。

`crates/prefab` 是可生成对象模板和 gameplay-facing 对象组合基础库。

它组合 ECS、physics、navigation、render 等数据，提供可以被 gameplay setup 直接生成的完整对象模板。
它也是外部 gameplay、intent、external_runtime、app 面向底层 ECS、physics、render、audio 能力的边界层；这些外部层不直接使用这些基础库。

音频能力由独立的 `crates/audio` 基础层负责。对象自身有哪些声音槽位由 `ecs::components::base::AudioClips` 描述，`prefab` 可以组合这些 ECS 数据，并注册从生命周期事件到 `audio::PlayAudioRequest` 的窄桥接系统。

## 代码落点

- 2D 世界对象 prefab：写到 `crates/prefab/src/world_2d`。
- 3D 世界对象 prefab：未来写到 `crates/prefab/src/world_3d`。
- 屏幕 UI prefab：写到 `crates/prefab/src/ui`。
- 2D 世界相机 prefab：写到 `crates/prefab/src/world_2d/camera.rs` 或 `world_2d/camera/`；世界相机是常规世界对象，不属于 demo。

## 骨架

```rust
use bevy::prelude::*;
use crate::Prefab;

pub struct YourObjectPrefab {
    pub position: Vec2,
}

#[derive(Bundle)]
pub struct YourObjectBundle {
    pub transform: Transform,
    pub visibility: Visibility,
}

impl Prefab for YourObjectPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(YourObjectBundle {
            transform: Transform::from_xyz(self.position.x, self.position.y, 0.0),
            visibility: Visibility::default(),
        }).id()
    }
}
```

## 边界规则

- `prefab` 可以依赖 `ecs`、`physics`、`navigation`、`audio`、`render_2d`、`interaction`。
- 对象模板需要声音资源配置时，优先组合 `ecs::components::base::AudioClips`。
- `prefab` 不硬编码具体资源路径；图片、音频、manifest 等资源路径由 `catalog` 绑定，再通过构造参数传入 prefab。
- 未来 3D prefab 可以依赖 `render_3d`。
- `prefab` 不读取键盘、鼠标、手柄、外设、AI、网络或脚本输入。
- `prefab` 不写底层 ECS system 函数；可以封装和导出 gameplay-facing spawn API 或窄 facade。
- `prefab` 不决定生成时机。
- `prefab` 不在一个 prefab 的 `spawn` 内部生成另一个 prefab；多个 prefab 的生成顺序由 gameplay 编排。
- `prefab` 可以给可交互对象挂 `interaction::InteractionAction`，但不处理点击后的业务。
- `prefab/src/ui` 只放具体 UI 画面 prefab，例如 menu；不要放 world camera prefab。
- 世界相机 prefab 属于 `prefab/src/world_2d` / `prefab/src/world_3d`，可以组合 `render_2d` / `render_3d` 暴露的 camera bundle。
- `gameplay` 注册 `PrefabPlugin`，`app` 不直接注册 `PrefabPlugin`、`EcsPlugin`、`PhysicsPlugin` 或 `Render2dPlugin`。
- `external_runtime`、`intent`、`gameplay` 使用 `prefab` 暴露的最小合法接口，不直接使用裸 `ecs`。
- `gameplay` 决定具体 gameplay session 使用哪些 prefab。
- `gameplay` 决定什么时候进入或退出 gameplay session。
- 如果 gameplay API 需要通过业务 ID 找到实体，prefab 应该组合 gameplay-facing id 组件，并暴露窄 facade 供 gameplay 查询。

## Bundle 规则

- 生成实体时优先使用 prefab bundle，不要在生成系统里散装组件。
- `commands.spawn((A, B, C))` 这种现场 tuple 组合不允许出现在 prefab；先定义命名 bundle/product，再 `spawn(named_bundle)`，子节点用 `with_children` 或明确的 render children helper。
- prefab 根实体必须一次性 `spawn(完整命名 bundle)`；优先让 `YourPrefab` 自己 `#[derive(Bundle)]` 并持有 root、render product、ecs/physics/audio 等字段。
- 不要用 `spawn(root_bundle).insert(render_bundle)` 这种后补组件方式表达对象结构；如果 prefab 的结构需要单独 bundle，必须有明确原因。
- 具体游戏可以添加 `CharacterPrefabBundle`、`EnemyPrefabBundle` 等对象模板。
- 具体 prefab 本身保存生成所需数据，优先暴露 prefab struct + bundle，并实现最小 `Prefab` trait。
- `Prefab` trait 只表达公共生成能力；具体 prefab 的特殊能力放在自己的类型或模块里。
- 不要把每个 prefab 做成 Bevy plugin。
- 模板本身不携带默认内容 prefab 或默认内容资源。

## Render 边界

- `prefab` 可以组合 `render_2d` 暴露的、挂在 Bevy Main World Entity 上的表现组件、marker 或 bundle。
- `prefab` 里的 render 组合只表达对象的表现数据或表现身份，不表示直接执行渲染。
- UI prefab 必须使用 `render_2d::products::ui` 暴露的 UI root bundle，不要散装未绑定 camera 的 root UI。
- UI camera 由 gameplay 使用 `render_2d::primitives::camera::UiCamera` 生成，再把 UI root 绑定到该 camera entity。
- UI prefab 公开 API 不暴露 Bevy `Entity` 或 camera target 句柄；camera 绑定由 gameplay 在 spawn 后插入 `UiTargetCamera(camera_entity)`。
- 屏幕 UI root 应该显式带 `UiTargetCamera(ui_camera)`；UI camera 可以带 `IsDefaultUiCamera` 作为默认 fallback。
- 世界 camera 应优先通过 prefab/catalog 生成，不要在 gameplay 里直接散装 render camera bundle。
- `prefab` 不直接操作 RenderApp、Render World、render graph、pipeline、GPU resource 或 `wgpu`。
- `prefab` 不把实体生成到 Render World；它只通过 `Commands` 生成 Main World Entity。
- Render SubApp 如何 extract、prepare、queue 和 draw，属于 Bevy/render 层，不属于 `prefab`。

## Audio 边界

- `prefab` 注册 `audio::AudioFoundationPlugin`，让默认 gameplay 组装具备播放固定音频资源的能力。
- `prefab` 可以提供窄桥接 system，把 ECS 事件和 `AudioClips` 转成 `audio::PlayAudioRequest`。
- `prefab` 不直接生成 Bevy `AudioPlayer`；真正播放由 `crates/audio` 处理。
- 具体对象使用哪些声音，例如 `object_action.wav`、`engine_loop`、`level_bgm`，属于 prefab 或未来 content。
- 具体音频资源路径属于 `catalog` 或未来 content catalog；`prefab` 只组合传入的声音槽位标识或句柄。
- `prefab` 不实现音频后端、DSP 合成器或播放 runtime。
- `prefab` 不决定什么时候播放声音；播放时机由 gameplay、ecs event 或其它上层流程决定。

## 验证要求

修改 `crates/prefab` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
