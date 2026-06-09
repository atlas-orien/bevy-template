# 3D 渲染子包

`render_3d` 是 3D 表现层。

它的任务是“把世界显示成 3D”，不是“决定世界怎么变化”。

## 职责

- 3D 相机。
- 3D 场景。
- 网格、材质、灯光。
- 3D 界面或 3D 专用表现连接代码。
- 读取 `ecs` 数据，把游戏世界显示出来。
- 创建渲染专用 Entity、Component 和动画状态。

## 当前状态

这个子包已经存在，但默认没有接入 `app`。

当前默认模板仍然运行 2D：

```rust
Render2dPlugin
```

需要做 3D 模板时，在顶层组装 `Render3dPlugin`。

## 不应该放这里

- 不写 2D 精灵或 2D 抬头显示。
- 不定义核心组件数据。
- 不读取输入。
- 不写入 intent。
- 不写世界模拟、移动、战斗、碰撞或物品结算。
- 不依赖 external_runtime、intent、prefab、physics、gameplay、render_2d。

`render_2d` 和 `render_3d` 应该保持独立。
