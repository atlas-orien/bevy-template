# UI prefab

这里放可生成的屏幕 UI prefab。

当前 demo 展示完整链路：

```text
gameplay
  -> 使用 render_2d::primitives::camera::UiCamera::default()
  -> 生成 UI 专用 camera entity
  -> 生成 DemoMenuPrefab
  -> 给 menu root 插入 UiTargetCamera(ui_camera)

DemoMenuPrefab (`demo_menu.rs`)
  -> 生成 root UI node
  -> 组合 render_2d::products::ui::DemoMenuVisual
  -> root 下挂 Start / Options / Quit 三个按钮
  -> 每个按钮组合 render_2d::products::ui::DemoMenuButtonVisual
  -> 每个按钮挂 InteractionAction

interaction crate
  -> 读取 Bevy Button Interaction
  -> 发出 InteractionEventMessage

gameplay/src/interaction
  -> match action id
  -> 执行具体业务
```

重要规则：

- UI camera 是屏幕 UI 专用 camera，不是 world camera。
- 生成顺序由 gameplay 编排；prefab 不在内部生成其它 prefab。
- UI root 必须在 gameplay spawn 编排里显式绑定 `UiTargetCamera(ui_camera)`。
- UI 节点本身不靠 `RenderLayers` 分层。
- UI 颜色、字体、尺寸、边距等视觉表现写在 `render_2d/src/products/ui`。
- 多个按钮通过不同 `InteractionAction` 区分。
- 点击后做什么不写在 prefab，写在 `gameplay/src/interaction`。
