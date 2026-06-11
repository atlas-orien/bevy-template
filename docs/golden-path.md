# 黄金路径：实体移动请求

这是“移动类行为”的端到端参考链路，用来帮助人理解请求如何穿过各层。

注意：本文是说明文档，不是项目硬约束来源。项目硬约束只在 `AI_PROTOCOL/*.md` 和 `crates/xtask/src/rules/*.rs`。

## 流程

```text
来源适配
-> peripherals 或 external_runtime 转成语义移动请求
-> manager::set_user_movement_intent(user, target) 或等价 gameplay request
-> gameplay::api::RuntimeRequestMessage::SetMovementIntent
-> gameplay::api::systems::forward_manager_requests_system
-> gameplay::api::systems::consume_gameplay_requests_system
-> prefab::identity::find_gameplay_entity(id)
-> intent::movement::set_movement_intent(entity, target, query)
-> 可选 navigation target/path/follower systems
-> ecs::systems::movement::movement_system
-> Transform 发生变化
-> render_2d 显示实体
```

## 当前架构说明

- 模板本身不生成默认对象。
- 具体项目决定哪个 prefab 拥有哪些 `GameplayEntityId`。
- 本机键盘、鼠标和手柄属于 `peripherals`。
- UI 和世界对象 hover/click 等 Bevy interaction 属于 `interaction`。
- AI、脚本、回放和未来网络属于 `external_runtime`。
- 当前链路中，来源适配层提交语义请求；不会直接接触 Bevy `Entity`、Commands、ECS component、physics 类型或 render 类型。
- `gameplay::api` 接收请求，并在这里把公开 id 映射回 Bevy entity 来执行。
- `intent` 只写入意图数据。
- 当对象需要基于路径移动时，`navigation` 拥有路径查询、路径目标和跟随数据。
- `ecs::systems` 执行真正改变 `Transform` 的世界规则。
- `render_2d` 读取世界状态并负责表现。

## 添加类似功能时的参考流程

1. 本机外设适配放到 `peripherals`；AI、脚本、回放等外部来源放到 `external_runtime::input`。
2. 新增或复用语义请求 API，由它提交 `RuntimeRequestMessage` 或等价 gameplay request。
3. 请求消息数据定义在 `gameplay::api::runtime_channel::message`。
4. 请求消费和执行放在 `gameplay::api::systems`。
5. 如果请求目标是某个实体，用户侧使用 runtime-facing id；内部再映射到 gameplay-facing id，不暴露裸 `Entity`。
6. 具体对象组合放到 `prefab`。
7. ECS 数据放到 `ecs::components`，全局状态放到 `ecs::resources`，世界规则放到 `ecs::systems`。
8. render、physics 和 navigation 是表现层或基础能力层，不是控制来源。

## 常见偏离

这些写法通常意味着代码偏离了当前分层。是否允许、如何处理，以 `AI_PROTOCOL/*.md` 和 `crates/xtask/src/rules/*.rs` 为准。

- 在 `gameplay`、`intent`、`ecs`、`prefab`、`render_2d` 或 `render_3d` 中读取键盘、鼠标、手柄、AI、脚本、回放或网络来源。
- 在 `prefab` 之外通过散装 component tuple 生成 gameplay entity。
- 通过 external runtime manager API 暴露裸 Bevy `Entity`。
- 在 `crates/physics` 之外直接 import Rapier。
