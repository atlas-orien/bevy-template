# AI 协议

这个文件写给 AI 代理。所有大写文件名都视为 AI 协议、约束或协作入口；普通 `README.md` 写给人类读者。

## 项目目标

这是一个 Bevy 游戏模板，作为 GitHub 模板或本地工作区使用，不发布到 crates.io。

AI 修改代码时必须优先保持目录职责清晰，让后续任务可以继续落在正确位置。

## 工作区分层

- `crates/error`: 统一错误、`Result<T>`、错误事件、严重级别、日志收集。
- `crates/components`: ECS 数据定义。只放组件、bundle、resource、marker、领域数据。
- `crates/controller`: 控制层。把键盘、手柄、AI、脚本等控制源转换成意图组件。
- `crates/simulation`: 模拟层。根据意图组件和规则修改世界。
- `crates/render_2d`: 2D 表现层。相机、屏幕、界面、精灵、纹理图集、2D 动画。
- `crates/render_3d`: 3D 表现层。相机、场景、网格、材质、灯光、3D 界面。
- `crates/app`: 最终组装层。配置 Bevy 插件、窗口和运行模式。
- `src/main.rs`: 根入口，只调用 app 子包。

## 依赖方向

- `components` 不依赖 `controller`、`simulation`、`render_2d`、`render_3d`。
- `controller` 可以依赖 `components` 和 `simulation` 的状态定义，但不直接做世界结算。
- `simulation` 可以依赖 `components`，负责真正改变世界。
- `render_2d` 和 `render_3d` 可以依赖 `components` 和 `simulation`，但不能把渲染职责写回它们。
- `app` 可以依赖所有需要组装的子包。
- 所有可失败逻辑都使用 `error::Result<T>`。

## 代码落点

- 新组件、bundle、resource、marker：写到 `crates/components`。
- 新玩家输入、手柄输入、AI 控制、脚本控制：写到 `crates/controller`。
- 新移动、战斗、交互、生成、销毁、状态流、世界规则：写到 `crates/simulation`。
- 新 2D 相机、精灵、纹理图集、2D 动画、HUD、菜单：写到 `crates/render_2d`。
- 新 3D 相机、场景、网格、材质、灯光：写到 `crates/render_3d`。
- 新错误类型、外部错误转换、错误事件处理：写到 `crates/error`。
- 插件组合、窗口设置、选择 2D 或 3D 运行模式：写到 `crates/app`。

## 命名规则

- 不要新增 `game_` 前缀的子包。
- 不要恢复 `ecs` 或 `gameplay` 子包名。
- 状态流使用 `simulation::flow::AppState`。
- 每个非 `error` 子包可以重新导出 `error::Result`，但不能定义自己的 `Result` 别名。
- 外部错误转换集中放在 `crates/error`，使用 `thiserror`。

## Bevy 规则

- 优先使用 Bevy plugin 作为功能边界。
- 组件只描述数据，不做行为。
- 系统负责行为。
- 需要在状态退出时清理的实体，应添加 marker component。
- 2D 和 3D 表现层保持独立。
- 默认应用组装 2D 表现层，不默认接入 `Render3dPlugin`。

## 当前演示

当前模板包含一个最小玩家演示：

- `components`: 定义 `Player`、`MovementIntent`、`Facing`、`PlayerSpeed`。
- `controller`: 读取 WASD 和方向键，写入 `MovementIntent`。
- `simulation`: 生成玩家实体，并根据意图移动 `Transform`。
- `render_2d`: 使用 Bevy 官方 `gabe-idle-run.png` 做 2D 跑步动画。

## 验证要求

交付前至少运行：

```sh
cargo fmt --check
cargo check --workspace
```

如果修改了可运行演示，尽量运行：

```sh
cargo run
```

如果无法运行，需要在回复中说明原因。
