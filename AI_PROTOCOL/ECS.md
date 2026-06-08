# ECS

这个文件是 `crates/ecs` 的 AI 规则。

`crates/ecs` 是 Bevy ECS 核心层，包含：

- `crates/ecs/src/components`: 挂在 Entity 上的组件数据。
- `crates/ecs/src/resources`: Bevy World 里的全局 ECS 数据。
- `crates/ecs/src/events`: 系统之间传递的事件数据。
- `crates/ecs/src/systems`: 读取和修改 ECS 数据的系统函数。

## 默认结构可以调整

当前目录结构是模板默认配置，不是不可修改的最终架构。

AI 可以根据具体游戏需求添加、修改或删除 `crates/ecs` 下的目录和模块，但必须遵守这些规则：

- 调整前先判断新结构是否让 ECS 职责更清楚。
- 不要为了保留模板形状而维护无用目录。
- 不要把 ECS 数据和 ECS 系统规则混到同一个文件里。
- 如果删除或改名目录，必须同步更新代码引用、Cargo 依赖和相关文档路径。
- 如果调整 `crates/ecs` 的结构，必须同步更新 `crates/ecs/README.md` 和本文件。

## 代码落点

- 新 `Component`、`Bundle`、marker：写到 `crates/ecs/src/components`。
- 新 `Resource`：写到 `crates/ecs/src/resources`。
- 新 `Event`：写到 `crates/ecs/src/events`。
- 新 ECS system 函数：写到 `crates/ecs/src/systems`。

## 边界规则

- `crates/ecs/src/components` 只描述实体数据，不做行为。
- `crates/ecs/src/resources` 只描述全局 ECS 数据，不加载文件资源。
- `crates/ecs/src/events` 只描述发生了什么，不处理后果。
- `crates/ecs/src/systems` 负责根据 ECS 数据执行世界规则。
- 不读取键盘、鼠标、手柄、网络输入；这些放到 `crates/controller`。
- 不写 sprite、动画、相机、UI 布局、材质、光照；这些放到渲染层。

## 命名规则

- 系统函数使用 `_system` 后缀，例如 `movement_system`、`damage_system`。
- 不要恢复独立的 `components` 或 `system` crate；ECS 数据和系统统一放在 `crates/ecs`。
- 不要新增 `game_` 前缀的 ECS 模块名。

## 当前演示

当前模板包含一个最小玩家演示：

- `crates/ecs/src/components`: 定义 `Player`、`MovementIntent`、`Facing`、`PlayerSpeed`。
- `crates/controller/src/keyboard`: 读取 WASD 和方向键，写入 `MovementIntent`。
- `crates/simulation`: 管理 Playing 状态下的玩家生成和系统调度。
- `crates/ecs/src/systems`: 未来用于放根据意图移动 `Transform` 的系统函数。
- `crates/render_2d`: 使用 Bevy 官方 `gabe-idle-run.png` 做 2D 跑步动画。

## 验证要求

修改 `crates/ecs` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
