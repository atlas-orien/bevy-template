# interaction

`interaction` 是 Bevy App 内部的交互事件桥接层。

Bevy UI 和 picking 负责命中测试与交互状态；本 crate 负责把这些 interaction 转换成项目语义 message。

第一版只处理 UI Button 的 click/hover 状态。未来可以接入 2D/3D world entity 的 pointer click、hover、drag 等事件。
