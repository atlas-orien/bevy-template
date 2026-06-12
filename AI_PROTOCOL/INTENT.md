此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# INTENT

这个文件是 `crates/intent` 的 AI 规则。

`crates/intent` 是 Entity 意图层。

它只表达一个 Entity “想做什么”，不表达这个想法来自哪里。

## 核心职责

- 提供写入 ECS intent 数据的语义 API。
- 表达可执行意图，例如移动、攻击、交互、使用物品。
- 设置 intent 时必须明确目标 `Entity`。
- 只作用于已经具备对应 intent 组件的 Entity；这些组件通常由 `prefab` 组合进可控制对象。
- 不决定世界结果，只表达“想做什么”。

## 代码落点

- 移动意图：写到 `crates/intent/src/movement`。

键盘、鼠标、手柄、UI、AI、脚本、网络等来源不属于 `intent`。来源应该先经过对应边界层，再调用 `intent` 提供的 API 写入 ECS intent 数据。

本机键盘、鼠标和手柄属于 `peripherals`。UI 和世界对象 hover/click 等 Bevy interaction 属于 `interaction`。AI、脚本、回放和未来网络等 Bevy App 外部来源属于 `external_runtime`。

## 边界规则

- 不生成实体。
- 不读取键盘、鼠标、手柄、外设、AI、脚本或网络来源。
- 不直接依赖或使用裸 `ecs`；通过 `prefab` 提供的最小合法接口写入 intent 数据。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不决定移动路径、速度、碰撞、到达判定等结算规则。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。
- 不定义 `Component`、`Bundle`、`Resource`、`Event`。

## 依赖规则

- `intent` 不依赖 `gameplay`；调度状态由其它层负责。
- `intent` 必须依赖 `error`。
- `intent` 可以依赖 `prefab`。
- `intent` 不依赖 `ecs`。
- `intent` 不依赖 `physics`。
- `intent` 不依赖 `audio`、`render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/intent` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
