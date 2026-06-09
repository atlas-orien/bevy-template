# 2D 渲染子包

`render_2d` 是 2D 表现层。

它的任务是“把世界显示出来”，不是“决定世界怎么变化”。

## 职责

- 2D 相机。
- 2D 屏幕、抬头显示、菜单、界面。
- 精灵、纹理图集、2D 动画。
- 读取 `ecs` 数据，把游戏世界显示出来。
- 创建渲染专用 Entity、Component 和动画状态。

## 当前结构

- `camera`: 2D 相机表现，`main_camera.rs` 放主相机 marker/bundle，`systems.rs` 放相机生成和同步 system。
- `animation`: 2D 表现层动画，分为 `frame` 帧动画和 `skeletal` 骨骼动画。
- `appearance`: 2D 外观属性，放颜色、透明度、可见性。
- `geometry`: 2D 表现层几何，放视觉形状、尺寸、锚点。
- `transform`: 2D 视觉 transform，放表现偏移、缩放、旋转。
- `ordering`: 2D 视觉排序。
- `sprite`: sprite 专用表现属性。
- `characters`: 角色 2D 表现，`character.rs` 放角色 sprite marker/config/bundle。
- `screens`: 屏幕表现，`clear_color.rs` 放屏幕背景色等屏幕级表现 system。
- `ui`: 2D UI 表现，`theme.rs` 放表现层颜色常量，`markers.rs` 放 UI marker。

## 文件规则

- 每个目录的 `mod.rs` 只做模块导出、re-export 和 Plugin 组装。
- 具体 Component、Bundle、system 拆到语义明确的文件里。
- 不新增 `common.rs`、`misc.rs` 这类含义模糊的文件。
- 默认 2D 主相机由 `camera` 在 startup 生成；prefab 和 gameplay 不生成主相机。

## 表现属性

这些目录只表达视觉表现，不表达物理或 gameplay 判定。

- `RenderShape2d`: 视觉形状，不代表 collider。
- `RenderSize2d`: 视觉尺寸，不代表 hitbox。
- `RenderAnchor2d`: 视觉锚点，不代表 gameplay 坐标。
- `RenderColor2d`: 表现层颜色，不代表阵营或游戏状态。
- `RenderOffset2d`: 视觉偏移，不改变玩法位置。
- `RenderScale2d`: 视觉缩放，不改变物理尺寸。
- `RenderRotation2d`: 视觉旋转，不改变 gameplay 规则。
- `RenderZIndex2d`: 视觉排序，不代表玩法优先级。
- `RenderVisibility2d`: 显示开关，不代表实体是否存在。
- `RenderOpacity2d`: 视觉透明度。
- `RenderFlip2d`: sprite 翻转。

碰撞、攻击范围、寻路区域、触发区域不要写在这里。

## animation

`animation` 只处理视觉动画，不处理玩法时序。

- `animation/frame`: sprite sheet、texture atlas、逐帧播放。
- `animation/skeletal`: 2D bone、skeleton、骨骼播放状态。
- 攻击前摇、技能窗口、硬直、combo、碰撞判定不放在 render animation。

## 和 ecs/intent/gameplay 的区别

- `crates/ecs/src/components` 定义玩家是什么。
- `crates/ecs/src/systems` 定义玩家位置等 ECS 数据如何被规则改变。
- `external_runtime` 读取 input/local、input/device、input/ai 等控制来源；网络是双向通信层，v2 单独设计。
- `intent` 表达 Entity 想做什么。
- `prefab` 把 render_2d 的表现 bundle 组合进完整对象模板。
- `gameplay` 负责状态流、gameplay session 生命周期和 system 调度。
- `render_2d` 只负责玩家看起来是什么样。

## 渲染实体

表现层可以创建渲染专用子实体。

例如：

```text
Gameplay Entity
└── Render Entity
```

玩法 Entity 的 `Transform` 表示世界位置，由 ECS system 修改。

渲染子 Entity 的 `Sprite`、贴图图集、scale、动画 timer 由 `render_2d` 修改。

这样可以避免为了显示效果直接改玩法 Entity 的核心数据。

## 不应该放这里

- 不定义核心游戏数据。
- 不读取键盘来决定 Entity 意图。
- 不写入 intent。
- 不执行战斗、碰撞、物品结算等世界规则。
- 不依赖 external_runtime、intent、prefab、physics、gameplay。
- 不放 3D 网格、3D 灯光、3D 相机。
