# peripherals

`peripherals` 是 Bevy App 内部的本机外设适配层。

Bevy/winit 负责底层键盘、鼠标、手柄和 UI interaction 接入；本 crate 只负责把这些原始输入转换成项目语义动作。

## 职责

- 读取 Bevy 输入资源和 UI interaction。
- 把本机外设输入转换成 gameplay-facing 语义请求。
- 保持设备细节不进入 gameplay、intent、ecs、prefab 或 render 层。

## 当前结构

- `keyboard`: 键盘输入适配。
- `mouse`: 鼠标输入适配。
- `gamepad`: 手柄输入适配。
- `ui`: Bevy UI interaction 适配。

网络、AI、脚本和回放不是本 crate 职责。
