# resources

`resources` 放 Bevy ECS 全局 `Resource` 数据类型。

这里的 resource 不是 `assets/` 文件，而是 Rust 代码里的全局 ECS 数据。

适合放这里：

- 世界配置：`WorldConfig`
- 运行会话：`GameSession`
- 当前关卡：`CurrentLevel`
- 难度设置：`Difficulty`
- 输入配置：`InputConfig`

文件规则：

- 按最小语义组拆文件，不按“一类型一文件”机械拆分。
- 一个文件只放同一语义组的 Resource。
- `world.rs`: 世界级配置，例如 `WorldConfig`。
- `session.rs`: 运行会话级全局数据，例如 `GameSession`。
- Resource 只描述 Bevy World 里的全局 ECS 数据，不负责加载磁盘资源文件。
- 注册默认 Resource 写在 `resources/mod.rs` 的 `ResourcesPlugin`。

不适合放这里：

- 图片、音频、字体、地图文件，这些放根目录 `assets/`
- 只属于某个实体的数据，这些应该是 `Component`
- 加载资源文件的系统函数
