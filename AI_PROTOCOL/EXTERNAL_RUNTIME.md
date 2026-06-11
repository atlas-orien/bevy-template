# EXTERNAL_RUNTIME

这个文件是 `crates/external_runtime` 的 AI 规则。

`external_runtime` 是 Bevy App 外部的 runtime 和 manager-side adapter 层。

项目运行时有两套系统：

- Bevy App：运行 Bevy `World`、`Schedule`、render、physics、gameplay。
- External Runtime：运行 Bevy App 外部的 AI、脚本、回放，以及未来网络等外部系统。

`external_runtime` 通过 `external_runtime::manager` 持有 gameplay transport，并通过双向 channel 和 Bevy App 通信。

## 核心职责

- 启动和停止 Bevy App 外部的 runtime loop。
- 持有 manager，作为外部系统进入 gameplay 的唯一入口。
- 管理 input/ai、script、replay 等 Bevy App 外部来源模块。
- 把外部来源转换成 manager API 调用。
- 不直接读取或修改 Bevy `World`。
- 本机键盘、鼠标、手柄和 UI interaction 属于 `crates/peripherals`，不属于 `external_runtime`。

## 代码落点

- runtime loop：写到 `crates/external_runtime/src/runtime`。
- Bevy App 外部到 gameplay 的通道组装：写到 `crates/external_runtime/src/bridge`。
- manager API：写到 `crates/external_runtime/src/manager`。
- App 外部来源总入口：写到 `crates/external_runtime/src/input`。
- AI 来源：写到 `crates/external_runtime/src/input/ai`。

网络不是 v1 的子模块。网络是双向通信层，v2 单独设计。

## Runtime 规则

- `external_runtime::runtime` 是 Bevy App 外部的 loop/runner。
- 可以使用 Tokio 或其它异步运行机制。
- 不直接读取或修改 Bevy `World`。
- 不直接使用 `Commands`、`Query` 或 `ResMut`。
- 不做成 Bevy `Plugin` 注册到 `App` 里。
- 必须通过 `external_runtime::manager` 和 request/update channel 进入 gameplay。

## External source adapter 规则

- external source adapter 只处理 Bevy App 外部来源，例如 AI、脚本、回放和未来网络。
- external source adapter 不读取本机键盘、鼠标、手柄或 UI interaction。
- external source adapter 只把外部来源转换成 manager API 调用，不直接生成实体。
- external source adapter 不直接使用裸 `ecs`，通过 manager API 提交 gameplay 请求或 intent 请求。
- external source adapter 不直接使用 `Commands`、`Query`、`Res`、`ResMut`、`Transform` 或物理组件。

## Manager 规则

- 用户和外部模块优先通过 manager API 操作 gameplay。
- 普通用户 API 使用 `RuntimeUserId` 或 `RuntimeObjectId` 表达外部可见对象身份。
- `GameplayEntityId` 是 manager 到 gameplay 的内部路由 id，只用于 manager state、runtime request/update message 和 gameplay 内部查找，不作为普通用户 API 参数。
- manager 负责把 `RuntimeUserId` / `RuntimeObjectId` 解析成内部 `GameplayEntityId`，再提交 `RuntimeRequestMessage`。
- `RuntimeRequestMessage` 是 external runtime 到 world 的内部请求，不应该被普通用户代码到处构造。
- `RuntimeUpdateMessage` 是 world 到 external runtime 的内部消息，不应该被普通用户代码到处构造。
- manager API 不向用户暴露 Bevy `Entity`。
- manager 必须有状态，可以维护 runtime-facing id 到 gameplay-facing id 的 registry，并允许用户按公开 id 查询。
- manager 不向用户暴露 Bevy `Entity`。
- manager 属于 `external_runtime`，不属于 `gameplay`。
- `manager/user.rs` 定义给用户和外部模块使用的高层 API；用户 API 优先写成纯函数式门面，内部接收 manager 并调用 manager 状态和 bridge。
- `manager/transport.rs` 定义 manager 内部使用的 request/update channel transport。
- 用户 API 不直接暴露 `RuntimeRequestSender`。
- transport 不应该被普通用户代码直接使用。
- `manager/transport.rs` 和 `manager/state.rs` 不对外公开；普通用户只能通过 `manager/user.rs` 和 `ExternalRuntimeManager` 进入。
- gameplay 不依赖 manager，也不调用 manager；gameplay 只向 update channel 发消息。
- manager 不进入 Bevy `World`，只向 request channel 发消息，并从 update channel 接收消息。
- channel 机制属于 `helper`，不属于 `external_runtime`。

## 边界规则

- 不定义核心 `Component`、`Bundle`、`Resource`、`Event`。
- 不生成实体；运行中生成必须通过 gameplay API 请求。
- 不直接依赖或使用裸 `ecs`。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。
- 不定义或导出 Bevy `Plugin`。

## 依赖规则

- `external_runtime` 可以依赖 `gameplay`，用于持有 request/update channel 端点和消息类型。
- `external_runtime` 可以依赖 `helper`，用于共享 channel/transport 基础设施。
- `external_runtime` 可以依赖 `intent` 和 `prefab`，但优先通过 manager API。
- `external_runtime` 可以依赖 `tokio`。
- `external_runtime` 必须依赖 `error`。
- `external_runtime` 不依赖 `audio`。
- `external_runtime` 不依赖 `ecs`。
- `external_runtime` 不依赖 `physics`。
- `external_runtime` 不依赖 `render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/external_runtime` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
