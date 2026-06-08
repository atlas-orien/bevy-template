# INPUT

这个文件是 `crates/input` 的 AI 规则。

`crates/input` 是输入来源适配层。

它读取外部来源，并把这些来源转换成 `crates/intent` 表达的 Entity 意图。

## 核心职责

- 读取键盘、鼠标、手柄、网络消息、服务端推送等外部来源。
- 找到这些输入应该控制的 `Entity`。
- 调用 `intent` crate 写入 Entity 意图。
- 不决定世界结果，只做来源到意图的转换。

## 代码落点

- 本地输入：写到 `crates/input/src/local.rs`。

未来需要网络输入、服务端推送、脚本输入或回放输入时，再按需求添加新模块。不要提前维护空结构。

## 边界规则

- 可以读取 `ButtonInput<KeyCode>`、鼠标、手柄等输入类型。
- 可以读取网络消息或服务端推送数据。
- 可以查询用于定位可控 Entity 的组件。
- 必须通过 `intent` 写入意图。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不定义核心 `Component`、`Bundle`、`Resource`、`Event`。
- 不生成实体。
- 不使用 prefab。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。

## 依赖规则

- `input` 可以依赖 `ecs`。
- `input` 可以依赖 `intent`。
- `input` 可以依赖 `simulation` 的状态定义。
- `input` 必须依赖 `error`。
- `input` 不依赖 `prefab`。
- `input` 不依赖 `physics`。
- `input` 不依赖 `render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/input` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
