# PREFAB

这个文件是 `crates/prefab` 的 AI 规则。

`crates/prefab` 是可生成对象模板和 gameplay-facing 对象组合基础库。

它组合 ECS、physics、render 等数据，提供可以被 gameplay setup 直接生成的完整对象模板。
它也是外部 gameplay、intent、external_runtime、app 面向底层 ECS、physics、render 能力的边界层；这些外部层不直接使用这些基础库。

## 代码落点

- 2D 世界对象 prefab：写到 `crates/prefab/src/world_2d`。
- 3D 世界对象 prefab：未来写到 `crates/prefab/src/world_3d`。
- 屏幕 UI prefab：写到 `crates/prefab/src/ui`。

## 边界规则

- `prefab` 可以依赖 `ecs`、`physics`、`render_2d`。
- 未来 3D prefab 可以依赖 `render_3d`。
- `prefab` 不读取键盘、鼠标、手柄、外设、AI、网络或脚本输入。
- `prefab` 不写底层 ECS system 函数；可以封装和导出 gameplay-facing spawn API 或窄 facade。
- `prefab` 不决定生成时机。
- `gameplay` 注册 `PrefabPlugin`，`app` 不直接注册 `PrefabPlugin`、`EcsPlugin`、`PhysicsPlugin` 或 `Render2dPlugin`。
- `external_runtime`、`intent`、`gameplay` 使用 `prefab` 暴露的最小合法接口，不直接使用裸 `ecs`。
- `gameplay` 决定具体 gameplay session 使用哪些 prefab。
- `gameplay` 决定什么时候进入或退出 gameplay session。
- 如果 gameplay API 需要通过业务 ID 找到实体，prefab 应该组合 gameplay-facing id 组件，并暴露窄 facade 供 gameplay 查询。

## Bundle 规则

- 生成实体时优先使用 prefab bundle，不要在生成系统里散装组件。
- 具体游戏可以添加 `Player2dPrefabBundle`、`Enemy2dPrefabBundle` 等对象模板。
- 具体 prefab 本身保存生成所需数据，优先暴露 prefab struct + bundle，并实现最小 `Prefab` trait。
- `Prefab` trait 只表达公共生成能力；具体 prefab 的特殊能力放在自己的类型或模块里。
- 不要把每个 prefab 做成 Bevy plugin。
- 模板本身不携带 demo prefab 或 demo 资源。

## Render 边界

- `prefab` 可以组合 `render_2d` 暴露的、挂在 Bevy Main World Entity 上的表现组件、marker 或 bundle。
- `prefab` 里的 render 组合只表达对象的表现数据或表现身份，不表示直接执行渲染。
- `prefab` 不直接操作 RenderApp、Render World、render graph、pipeline、GPU resource 或 `wgpu`。
- `prefab` 不把实体生成到 Render World；它只通过 `Commands` 生成 Main World Entity。
- Render SubApp 如何 extract、prepare、queue 和 draw，属于 Bevy/render 层，不属于 `prefab`。

## 验证要求

修改 `crates/prefab` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
