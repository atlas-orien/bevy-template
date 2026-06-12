# peripherals

`peripherals` 是 Bevy App 内部的本机外设适配层。

Bevy/winit 负责底层键盘、鼠标和手柄接入；本 crate 只负责把这些原始输入转换成项目语义动作。

## 职责

- 读取 Bevy 输入资源。
- 把本机外设输入转换成项目语义动作。
- 用 `LocalInputContext` 区分菜单、角色场景、文本输入等输入语境。
- 用 `LocalInputAction` 表达 UI 导航、移动、交互、暂停等设备无关动作。
- 保持设备细节不进入 gameplay、intent、ecs、prefab 或 render 层。

## 当前结构

- `local_input`: 本机输入上下文和设备无关语义动作。
- `keyboard`: 键盘输入适配。
- `mouse`: 鼠标输入适配。
- `gamepad`: 手柄输入适配。

Bevy interaction、网络、AI、脚本和回放不是本 crate 职责。

## 扩展原则

`KeyCode` 只表示物理按键，不直接表示游戏意图。

新增输入时，优先按以下链路扩展：

```text
KeyCode / MouseButton / GamepadButton
-> LocalInputAction
-> 按 LocalInputContext 路由到 interaction / gameplay / intent 边界
```

例如，同一个方向键在 UI 菜单中可以路由为 `UiNavigationInputMessage`，在角色场景中可以路由为移动意图。具体角色如何移动、菜单如何执行业务，不写在 `peripherals`。
