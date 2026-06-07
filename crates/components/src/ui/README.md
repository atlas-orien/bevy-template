# ui

`ui` 放 UI 相关 ECS 数据。

适合放这里：

- HUD 根节点 marker：`HudRoot`
- 菜单 marker：`MainMenuRoot`、`PauseMenuRoot`
- 血条数据：`HealthBar`
- 名字牌数据：`Nameplate`
- 按钮动作：`ButtonAction`
- UI 和实体的绑定关系：例如血条绑定到哪个角色

不适合放这里：

- 创建 UI 节点的系统
- UI 布局、颜色、字体、图片
- 按钮点击后的业务逻辑
- 2D/3D 具体显示方式
