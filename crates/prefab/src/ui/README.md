# UI prefab

这里放可生成的屏幕 UI prefab。

当前 demo 展示完整链路：

```text
UiCameraPrefab
  -> 生成 UI 专用 Camera2d
  -> 使用 render_2d::camera::ui_camera_bundle()
  -> 带 IsDefaultUiCamera
  -> Camera.order = UI_CAMERA_ORDER

DemoMenuPrefab
  -> 接收 gameplay 传入的 ui_camera Entity
  -> 生成 root UI node
  -> 组合 render_2d::ui::demo_menu_root_node()
  -> root 带 UiTargetCamera(ui_camera)
  -> root 下挂 Start / Options / Quit 三个按钮
  -> 每个按钮组合 render_2d::ui::demo_menu_button_node(label)
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
- UI root 必须显式绑定 `UiTargetCamera(ui_camera)`。
- UI 节点本身不靠 `RenderLayers` 分层。
- UI 颜色、字体、尺寸、边距等视觉表现写在 `render_2d/src/ui/menu.rs`。
- 多个按钮通过不同 `InteractionAction` 区分。
- 点击后做什么不写在 prefab，写在 `gameplay/src/interaction`。
