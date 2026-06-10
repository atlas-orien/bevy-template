# navigation

`navigation` 是项目导航基础层。

它负责 Bevy World 内的路径、导航代理、目标、路径查询和路径跟随。它不读取外部输入，也不负责决定“为什么要移动到那里”。

## 当前结构

- `agent`: 导航代理组件。
- `target`: 导航目标组件。
- `path`: 路径数据组件。
- `query`: 路径查询和算法。
- `systems`: 路径刷新和路径跟随系统。

## 2D / 3D

这是一个 crate，不拆成两个 crate。

维度相关类型显式带后缀：

- `NavigationAgent2d`
- `NavigationAgent3d`
- `NavigationTarget2d`
- `NavigationTarget3d`
- `NavigationPath2d`
- `NavigationPath3d`

第一版只提供直线路径和跟随边界，后续可以在 `query` 中替换为 grid、waypoint graph 或 navmesh。

## 边界

- `external_runtime` 只产生目标或请求。
- `intent` 只表达移动意图。
- `navigation` 把目标转换成路径，并推动路径跟随。
- `prefab` 决定哪些对象组合导航能力。
- `render_2d` / `render_3d` 只做导航可视化。
