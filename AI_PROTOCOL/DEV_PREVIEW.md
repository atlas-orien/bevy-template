此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# DEV_PREVIEW

这个文件是 `crates/dev_preview` 的 AI 规则。

`crates/dev_preview` 是纯代码开发预览台，用来替代传统编辑器里的预览窗口。

它只服务开发期验证：当新增组件、bundle、prefab、UI、相机、渲染表现或交互链路时，可以先在这里写一个小预览并运行观察。

## 核心职责

- 提供可运行的开发预览入口。
- 按预览场景组织示例，例如 UI 菜单、组件显示、prefab 生成、物理体、相机、输入链路。
- 允许组合正式 crate 暴露的 API，帮助 AI 快速验证新代码。
- 保持预览代码和正式游戏逻辑隔离。

## 代码落点

- 入口写到 `crates/dev_preview/src/main.rs`。
- 预览场景写到 `crates/dev_preview/src/previews`。
- 每个预览场景一个文件，例如 `previews/demo_menu.rs`。
- `previews/mod.rs` 只负责声明模块和分发预览，不写具体 Bevy 预览逻辑。

## 边界规则

- `dev_preview` 是最外层开发工具 crate。
- 正式游戏 crate 不允许依赖 `dev_preview`。
- `dev_preview` 可以依赖正式 crate，用来组装开发预览。
- 预览代码不能成为正式 gameplay 逻辑的唯一实现。
- 预览里发现需要复用的正式能力时，把正式能力移到所属 crate，再由 `dev_preview` 调用。
- 不要把 `dev_preview` 当成测试替代品；单元测试、集成测试仍然写在对应 crate。
- 不要把长期 gameplay 状态、正式 spawn plan 或正式业务规则放到 `dev_preview`。

## 运行方式

默认运行 demo 菜单预览：

```sh
cargo run -p dev_preview
```

指定预览：

```sh
cargo run -p dev_preview -- demo_menu
```

## 验证要求

修改 `crates/dev_preview` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
