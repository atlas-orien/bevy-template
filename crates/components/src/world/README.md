# world

`world` 放世界、地图、关卡相关 ECS 数据。

适合放这里：

- 世界配置：`WorldConfig`
- 关卡 marker：`LevelMarker`
- 出生点：`SpawnPoint`
- 区域：`Zone`
- 障碍物：`Obstacle`
- 传送点：`Portal`

不适合放这里：

- 关卡加载系统
- 出生实体系统
- 碰撞和寻路逻辑
- 背景图片或地图渲染
