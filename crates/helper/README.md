# helper

`helper` 是跨 runtime / 跨 crate 的共享基础设施层。

它不属于 Bevy App，也不属于 `external_runtime`。两个世界之间都需要使用的通信能力放这里。

## 当前结构

- `channel`: channel 基础实现。

## 不应该放这里

- 不保存游戏状态。
- 不生成 Entity。
- 不写 gameplay、physics、render、prefab、state 或 intent 逻辑。
