此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# NAVIGATION

这个文件是 `crates/navigation` 的 AI 规则。

`crates/navigation` 是项目导航基础层，负责 Bevy World 内的路径、导航代理和路径跟随能力。

它不是输入来源层，也不是 Bevy App 入口层。用户点击地面属于 `peripherals` 的本机外设适配；AI 选择目标、脚本请求移动到某处属于 `external_runtime` 的来源适配；navigation 只处理进入 World 后“如何从当前位置走向目标”。

## 代码落点

- 导航插件入口：写到 `crates/navigation/src/plugin.rs`。
- 导航代理组件：写到 `crates/navigation/src/agent`。
- 导航目标组件：写到 `crates/navigation/src/target`。
- 路径数据组件：写到 `crates/navigation/src/path`。
- 路径查询和算法：写到 `crates/navigation/src/query`。
- 路径跟随系统：写到 `crates/navigation/src/systems`。

## 2D / 3D 规则

- `navigation` 是一个 crate，不拆成 `navigation_2d` 和 `navigation_3d`。
- 维度相关类型必须显式带 `2d` 或 `3d` 后缀，例如 `NavigationAgent2d`、`NavigationAgent3d`、`NavigationPath2d`、`NavigationPath3d`。
- 维度无关配置才使用通用命名。
- 第一版只提供直线路径查询和路径跟随边界；不要假装已经有完整 navmesh 或复杂避障 runtime。

## 边界规则

- 可以定义导航相关 `Component`、`Resource` 和 system。
- 可以读取 `Transform`、导航目标、路径数据并更新导航路径。
- 可以写入 `Transform` 来执行路径跟随。
- 不读取键盘、鼠标、手柄、外设、AI、脚本或网络来源。
- 不生成 gameplay 实体；对象是否拥有导航能力由 `prefab` 组合。
- 不依赖 `external_runtime`、`intent`、`gameplay`、`prefab`、`render_2d`、`render_3d` 或 `physics`。
- 不封装物理后端。
- 不写 sprite、动画、UI、相机或调试绘制；导航可视化放在 render crate 的 debug/overlay 目录。

## 和现有移动链路

`peripherals` 和 `external_runtime` 可以通过对应边界提交移动目标。

`intent` 只表达 Entity 想移动到哪里。

`navigation` 在 Bevy World 内把目标转换成路径和下一步方向。

`ecs` 或 `navigation` 的跟随系统执行最终 Transform 变化。第一版 navigation 自带简单 follower，后续如果移动规则变复杂，可以再把 follower 与 ECS movement 规则明确拆分。

## 依赖规则

- `navigation` 可以依赖 `bevy`。
- `navigation` 必须依赖 `error`。
- `navigation` 不依赖 `external_runtime`。
- `navigation` 不依赖 `intent`。
- `navigation` 不依赖 `gameplay`。
- `navigation` 不依赖 `prefab`。
- `navigation` 不依赖 `physics`。
- `navigation` 不依赖 `render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/navigation` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
