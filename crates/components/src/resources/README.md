# resources

`resources` 放 Bevy ECS 全局 `Resource` 数据类型。

这里的 resource 不是 `assets/` 文件，而是 Rust 代码里的全局 ECS 数据。

适合放这里：

- 游戏配置：`GameConfig`
- 当前关卡：`CurrentLevel`
- 运行会话：`GameSession`
- 难度设置：`Difficulty`
- 输入配置：`InputConfig`

不适合放这里：

- 图片、音频、字体、地图文件，这些放根目录 `assets/`
- 只属于某个实体的数据，这些应该是 `Component`
- 加载资源文件的系统函数
