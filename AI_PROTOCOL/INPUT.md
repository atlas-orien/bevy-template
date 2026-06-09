# INPUT

这个文件是 `crates/input` 的 AI 规则。

`crates/input` 是输入来源适配层。

它读取外部控制来源，并把这些来源转换成项目内部能理解的语义：

- 持续性 Entity 行为：转换成 `crates/intent` 表达的 Entity 意图。
- 一次性高层玩法请求：转换成 `gameplay::api` 暴露的 gameplay 请求。

`input` 统一的是来源，不统一世界结果。

## 核心职责

- 读取键盘、鼠标、手柄、网络消息、服务端推送、外设、脚本、回放等外部控制来源。
- 把持续性角色行为转换成 `intent`，例如移动、瞄准、攻击准备。
- 把一次性高层请求转换成 `gameplay::api`，例如生成对象、切换状态、加载关卡。
- 需要写入 intent 时，找到这些输入应该控制的 `Entity`。
- 不决定世界结果，只做来源到项目语义的转换。

## 代码落点

- 本地输入：写到 `crates/input/src/local.rs`。
- 调用 gameplay API 的窄转发入口：写到 `crates/input/src/gameplay_api.rs`。

未来需要网络输入、服务端推送、脚本输入或回放输入时，再按需求添加新模块。不要提前维护空结构。

## 边界规则

- 可以读取 `ButtonInput<KeyCode>`、鼠标、手柄等输入类型。
- 可以读取网络消息或服务端推送数据。
- 可以通过 `prefab` 提供的查询类型定位可控 Entity。
- 持续性 Entity 行为必须通过 `intent` 写入意图。
- 一次性高层玩法请求必须通过 `gameplay::api` 提交。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不定义核心 `Component`、`Bundle`、`Resource`、`Event`。
- 不生成实体。
- 不直接依赖或使用裸 `ecs`；需要可控对象查询时使用 `prefab` 暴露的最小合法接口。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。

## 依赖规则

- `input` 可以依赖 `gameplay`，但只能使用 `gameplay::api` 作为高层请求边界。
- `input` 可以依赖 `intent`。
- `input` 可以依赖 `prefab`。
- `input` 必须依赖 `error`。
- `input` 不依赖 `ecs`。
- `input` 不依赖 `physics`。
- `input` 不依赖 `render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/input` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
