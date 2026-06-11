# Assets

`assets/` 只放 Bevy runtime 直接加载的成品资源。

代码里使用的路径相对 `assets/`：

```rust
asset_server.load("2d/animated/characters/name/name.png")
```

原始素材、参考资料、用户输入和工具输入放到 `workbench/`，不要在 runtime 代码里加载 `workbench/`。

## 基本规则

- 文件和目录使用 lowercase kebab-case。
- 路径尽量稳定，避免无意义重命名。
- 这里只放 runtime-ready 文件。
- 第三方资源来源和许可证记录写到 `docs/assets.md`。

## 目录

```text
assets/
  2d/
    animated/
    static/
  3d/
  ui/
  audio/
  fonts/
  shaders/
  cursors/
  branding/
  data/
  levels/
  scenes/
  platform/
```

## 2D

帧动画资源放到：

```text
assets/2d/animated/{category}/{name}/
  {name}.png
  {name}.frames.ron
```

静态 2D 图片放到：

```text
assets/2d/static/{category}/{name}/
  {name}.png
```

常用分类：

```text
backgrounds/
characters/
effects/
items/
particles/
props/
tilemaps/
```

## 3D

3D 资源按资源类型放到：

```text
assets/3d/
  models/
  textures/
  materials/
  animations/
  rigs/
  skeletons/
  scenes/
  environment-maps/
  lightmaps/
  irradiance-volumes/
  volumes/
```

runtime 3D 模型优先使用 `.glb`。

## Shaders

自定义 shader 放到：

```text
assets/shaders/
  2d/
  3d/
  ui/
  post-process/
  includes/
```

不会写 shader 时可以忽略这个目录。

## 其它

- `assets/ui`: 屏幕空间 UI 资源。
- `assets/audio`: 音频文件。
- `assets/fonts`: 字体。
- `assets/cursors`: 鼠标光标。
- `assets/branding`: 图标、logo、启动图。
- `assets/data`: runtime 数据文件。
- `assets/levels`: 关卡数据。
- `assets/scenes`: runtime 场景数据。
- `assets/platform`: 平台特定资源。
