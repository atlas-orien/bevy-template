# prefab

`prefab` 放可生成的游戏对象模板和面向 gameplay 的对象组合入口。

它负责把 `crates/ecs` 的游戏语义数据、`crates/physics` 的物理能力、`crates/navigation` 的导航能力、`crates/render_2d` 的表现数据和 `crates/audio` 的播放能力组合成可以直接生成的完整对象模板。
外部 gameplay 层不直接使用 `ecs`、`physics`、`navigation`、`render_2d`、`audio` 这些基础库，而是通过这里提供的封装入口使用它们。

## 职责

- 定义可生成对象的组合 Bundle。
- 组合 ECS、physics、render bundle 或配置。
- 注册音频基础插件，并把 ECS 音频槽位事件桥接成播放请求。
- 提供最小 `Prefab` trait，表达 prefab 实例可以生成主 Entity。
- 给 gameplay setup 提供稳定的对象生成入口。
- 给 gameplay 提供必要的窄 facade，让 gameplay 不直接依赖底层 crate。
- 给 gameplay 提供 gameplay-facing id 到 Bevy `Entity` 的查询 facade。

具体游戏应该在这里添加自己的对象模板，而不是在 gameplay setup 或生成系统里散装很多组件。

## 当前结构

- `world_2d`: 2D 世界对象 prefab，例如角色、物品、地图物件。
- `world_3d`: 3D 世界对象 prefab，当前只保留命名空间。
- `ui`: 屏幕 UI prefab，2D 和 3D 游戏都可以复用。
- `identity`: gameplay-facing id 和 Bevy `Entity` 的查询 facade。
- `lifecycle`: 面向 gameplay 的生命周期窄 facade。
- `navigation`: 面向 gameplay 的导航能力组合和窄 facade。

## 边界

- 可以依赖 `audio`、`ecs`、`physics`、`navigation`、`render_2d`。
- 未来 3D prefab 可以依赖 `render_3d`。
- 不读取输入。
- 不写底层 ECS system 函数；可以封装并导出 gameplay-facing spawn API 或窄 facade。
- 不负责状态流、关卡流程或生成时机。

`gameplay` 决定具体 gameplay session 使用哪些 prefab，以及什么时候进入或退出这些 session。

## render 边界

`prefab` 可以组合 `render_2d` 暴露的 Main World 表现组件、marker 或 bundle，例如角色 sprite 表现 bundle。

但 `prefab` 不直接操作 Bevy RenderApp、Render World、render graph、pipeline、GPU resource 或 `wgpu`。Render SubApp 如何抽取和绘制这些数据，属于 Bevy/render 层。

## audio 边界

`prefab` 可以组合 `ecs` 暴露的 `AudioClips`，表达某个对象有哪些可用音频资源。

`PrefabPlugin` 注册 `AudioFoundationPlugin`，并提供窄桥接系统：当实体生命周期事件发生时，读取实体的 `AudioClips`，发送 `PlayAudioRequest`。

具体对象使用哪些声音属于 prefab 或未来 content；音频播放 runtime 和复杂声音合成不放在 prefab。
