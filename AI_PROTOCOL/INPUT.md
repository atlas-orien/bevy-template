# INPUT

这个文件是 `crates/input` 的 AI 规则。

`crates/input` 是输入来源适配层。

它读取外部控制来源，并把这些来源转换成项目内部能理解的语义：

- 持续性 Entity 行为：转换成 `crates/intent` 表达的 Entity 意图。
- 一次性高层玩法请求：转换成 `gameplay::api` 暴露的 gameplay 请求。

`input` 统一的是来源，不统一世界结果。

## 核心职责

- 读取键盘、鼠标、手柄、外设、AI、脚本、回放等外部控制来源。
- 把持续性角色行为转换成 `intent`，例如移动、瞄准、攻击准备。
- 把一次性高层请求转换成 `gameplay::api`，例如生成对象、切换状态、加载关卡。
- 需要写入 intent 时，找到这些输入应该控制的 `Entity`。
- 为 Bevy 外部输入来源提供自己的 runtime loop。
- 不决定世界结果，只做来源到项目语义的转换。

## 代码落点

- 本地输入：写到 `crates/input/src/local`。
- 外设输入：写到 `crates/input/src/device`。
- AI 控制输入：写到 `crates/input/src/ai`。
- input 自己的运行循环：写到 `crates/input/src/runtime`。
- 外部 input runtime 到 Bevy App 的通道：写到 `crates/input/src/bridge`。
- 调用 gameplay API 的窄转发入口：写到 `crates/input/src/gameplay_api.rs`。

`input` 必须按来源域分目录，不要把大量来源都塞进一个文件。

具体来源域内部再按协议、设备、控制模型继续拆分。

网络不是 `input` 的子目录。网络是双向通信层，v2 需要单独设计 crate。

## Runtime 规则

- `input::runtime` 是外部输入来源的 loop/runner。
- `input::runtime` 可以使用 Tokio 或其它异步运行机制。
- `input::runtime` 不直接读取或修改 Bevy `World`。
- `input::runtime` 不直接使用 `Commands`、`Query` 或 `ResMut`。
- `input::runtime` 必须通过 bridge/channel 把外部输入送进 Bevy App。
- Bevy App 内部再把 bridge 中的数据转换成 `intent` 或 `gameplay::api` 请求。
- 不要把 `input` 做成 Bevy `Plugin` 注册到 `App` 里。

## 边界规则

- 可以读取 `ButtonInput<KeyCode>`、鼠标、手柄等输入类型。
- 可以通过 `prefab` 提供的查询类型定位可控 Entity。
- 持续性 Entity 行为必须通过 `intent` 写入意图。
- 一次性高层玩法请求必须通过 `gameplay::api` 提交。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不定义核心 `Component`、`Bundle`、`Resource`、`Event`。
- 不生成实体。
- 不直接依赖或使用裸 `ecs`；需要可控对象查询时使用 `prefab` 暴露的最小合法接口。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。
- 不定义或导出 `InputPlugin`。

## 依赖规则

- `input` 可以依赖 `gameplay`，但只能使用 `gameplay::api` 作为高层请求边界。
- `input` 可以依赖 `intent`。
- `input` 可以依赖 `prefab`。
- `input` 可以依赖 `tokio`，用于外部 input runtime。
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
