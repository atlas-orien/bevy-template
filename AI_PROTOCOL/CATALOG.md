此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# CATALOG

这个文件是 `crates/catalog` 的 AI 规则。

`crates/catalog` 是默认内容目录，负责把 runtime 资源路径、atlas layout、加载设置和 prefab 绑定成可生成的成品。

## 核心职责

- 保存模板默认资源路径。
- 使用 `AssetServer`、`Assets<TextureAtlasLayout>` 等 Bevy 资源加载能力创建带资源的 prefab。
- 给 `gameplay`、`dev_preview` 这类上层代码提供清晰入口，例如 `demo_player(...)`、`demo_ground(...)`、`demo_skeleton(...)`。
- 让 `gameplay` 只决定生成时机、位置和流程，不手写具体资源路径。

## 边界规则

- `catalog` 可以依赖 `bevy`、`prefab`、`render_2d`、`error`。
- `catalog` 不定义 ECS component、bundle、resource、message 或 gameplay system。
- `catalog` 不 spawn entity，不接收 `Commands`，不决定生成时机。
- `catalog` 不读取输入、网络、外部 runtime 或 gameplay 状态。
- `catalog` 只返回 prefab 或 prefab 所需的资源绑定数据。
- 具体对象结构仍属于 `prefab`；视觉 bundle/system 仍属于 `render_2d`。

## 资源规则

- runtime 资源路径常量写在 `catalog`，不要散在 `gameplay` 或 `dev_preview`。
- `gameplay` 和 `dev_preview` 不应该为具体 prefab 手写 `asset_server.load("...")`。
- 如果资源需要 loader settings，例如 tilemap array layout，也由 `catalog` 统一封装。

## 验证要求

修改 `crates/catalog` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
