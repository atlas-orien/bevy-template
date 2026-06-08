# CONTROLLER

这个文件是 `crates/controller` 的 AI 规则。

`crates/controller` 是控制来源到 intent 数据的转换层。

## 核心职责

- 读取键盘、鼠标、手柄、AI、脚本、网络等控制来源。
- 把控制来源转换成 ECS intent 组件。
- 不决定世界结果，只表达“想做什么”。

## 代码落点

- 键盘和鼠标输入：写到 `crates/controller/src/keyboard`。
- 手柄输入：写到 `crates/controller/src/gamepad`。
- AI 决策：写到 `crates/controller/src/ai`。
- 脚本、剧情、触发器：写到 `crates/controller/src/script`。
- 网络消息：写到 `crates/controller/src/network`。

## 边界规则

- 不生成实体。
- 不使用 prefab。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。
- 不定义 `Component`、`Bundle`、`Resource`、`Event`。

## 依赖规则

- `controller` 可以依赖 `ecs`。
- `controller` 可以依赖 `simulation` 的状态定义。
- `controller` 必须依赖 `error`。
- `controller` 不依赖 `prefab`。
- `controller` 不依赖 `physics`。
- `controller` 不依赖 `render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/controller` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
