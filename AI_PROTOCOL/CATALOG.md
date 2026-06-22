此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# CATALOG

这个文件是 `crates/catalog` 的 AI 规则。

`crates/catalog` 是默认内容目录，负责把 runtime 资源路径、atlas layout、加载设置和 prefab 绑定成可生成的具体内容对象。

`catalog` 是 `gameplay`、`dev_preview` 等上层代码面向默认内容的入口；`prefab` 是它背后的对象生成模板库。

## 核心职责

- 定义具体内容对象，例如 `WorldCamera2d`、`DemoPlayer`、`DemoGround`、`DemoSensorZone`。
- 在具体内容对象里保存对应 runtime 资源路径、loader settings 和默认内容参数。
- 使用 `AssetServer`、`Assets<TextureAtlasLayout>` 等 Bevy 资源加载能力创建带资源的 prefab。
- 给 `gameplay`、`dev_preview` 这类上层代码提供清晰入口，例如 `DemoPlayer::at(...).prefab(asset_server)`。
- 让 `gameplay` 只决定生成时机、位置和流程，不手写具体资源路径。

## 代码落点

- 2D 世界内容：写到 `crates/catalog/src/world_2d`。
- 3D 世界内容：未来写到 `crates/catalog/src/world_3d`。
- 默认 2D / 3D 世界相机内容写到对应 `world_2d` / `world_3d`，不要归入 demo 内容。
- 不天然属于 2D / 3D 世界对象的共享内容，例如 demo BGM：写到对应语义目录，例如 `crates/catalog/src/audio`。
- 聚合导出可以写到 `crates/catalog/src/demo.rs`，但具体内容类型不要全部堆在 `demo.rs`。
- 不创建 `dev_preview` 目录；开发预览应该使用 catalog 内容，预览自身的组合逻辑放在 `crates/dev_preview`。

## 边界规则

- `catalog` 可以依赖 `bevy`、`prefab`、`render_2d`、`render_3d`、`error`。
- `catalog` 不定义 ECS component、bundle、resource、message 或 gameplay system。
- `catalog` 不 spawn entity，不接收 `Commands`，不决定生成时机。
- `catalog` 不读取输入、网络、外部 runtime 或 gameplay 状态。
- `catalog` 只返回 prefab 或 prefab 所需的资源绑定数据。
- 具体对象结构仍属于 `prefab`；视觉 bundle/system 仍属于 `render_2d`。
- 3D animation set、animation state、clip wrapper、播放系统和状态同步系统属于 `render_3d`，不属于 `catalog`。
- `catalog` 不直接 import `render_3d::capabilities::animation::*` 来组装动画结构；需要 3D 动画时调用 `render_3d` 暴露的具体 product/capability constructor。
- `render_3d::primitives::materials/presets` 可以封装模板自带 preview/demo 材质资源；这类材质资源不需要再经由 catalog 二次包装。
- `catalog` 不直接定义通用 `NpcPrefab`、`PlayerPrefab` 等对象模板；这些放到 `prefab`。
- `catalog` 定义的是具体内容，例如 `Npc1`、`Npc2`、`DemoPlayer`，并在内部转换成 `prefab` 暴露的模板。
- `catalog` 可以定义默认相机内容，例如 `WorldCamera2d`，并转换成通用 camera prefab。

## 资源规则

- runtime 资源路径常量写在对应具体内容对象文件里，不要散在 `gameplay` 或 `dev_preview`。
- 不新增集中式 `paths.rs`、`resources.rs`、`assets.rs` 来收集所有路径；这会随着内容增长变成全局杂物文件。
- `gameplay` 和 `dev_preview` 不应该为具体 prefab 手写 `asset_server.load("...")`。
- 如果资源需要 loader settings，例如 tilemap array layout，也由 `catalog` 统一封装。
- 3D glTF 的 scene/model 路径可以写在具体 catalog 对象里；glTF animation label 到项目动画状态的映射属于 `render_3d` 具体 product/capability，不在 catalog 里另写 `Demo*Animations` 之类结构。

## 验证要求

修改 `crates/catalog` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
