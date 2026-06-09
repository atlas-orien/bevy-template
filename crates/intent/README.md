# intent

`intent` 是 Entity 意图层。

它只描述一个 Entity “想做什么”，不描述这个想法来自哪里。

键盘、手柄、AI、脚本、网络都不是 `intent` 本身。它们属于 `input` 或其它来源适配层，转换成 intent 后再写入可控制 Entity 的意图数据。

## 职责

- 提供写入 ECS intent 数据的语义 API。
- 明确是哪一个 `Entity` 想做什么。
- 写入 `MovementIntent`、`AttackIntent`、`UseItemIntent` 等 intent 数据。
- 只作用于已经具备对应 intent 组件的 Entity；这些组件通常由 `prefab` 组合进可控制对象。
- 不计算移动结果，不修改位置、血量、背包等世界结果。

## 当前结构

- `movement`: 移动意图，例如“这个 Entity 想往某个方向移动”或“想移动到某个位置”。

输入来源放在 `crates/input`。无论 input 来自键盘、手柄、AI 还是网络，都需要先转换成 intent。

## 和 gameplay 的区别

`intent` 表达“想做什么”。

`gameplay` 决定处理 intent 的 ECS system 在什么状态、什么阶段被调度。

## 和 ecs/systems 的区别

`intent` 写入 intent 数据。

`crates/ecs/src/systems` 读取 intent 数据并改变世界。

例如：

```text
外部来源转换出移动方向
intent 给某个 Entity 写入 MovementIntent
ecs/systems/movement 根据 MovementIntent + Speed 修改 Transform
```

## 不应该放这里

- 不生成实体。
- 不读取键盘、鼠标、手柄、网络输入。
- 不直接依赖或调用裸 `ecs`；通过 `prefab` 暴露的最小合法接口写入 intent 数据。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不决定怎么移动、怎么攻击、怎么结算；这些属于 `crates/ecs/src/systems`。
- 不封装物理后端。
- 不写渲染、动画、UI、相机。
- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
