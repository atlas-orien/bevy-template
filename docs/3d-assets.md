# 3D 游戏资源入门

这篇文档面向**第一次做 3D 游戏、不清楚需要哪些原始资源**的人。

目标是回答一个问题：

> 一个 3D 游戏，到底需要哪些 asset 才能跑起来？

读完你应该能看懂 `assets/3d/` 里每个目录是干什么的、为什么这样分、以及这些资源是怎么一步步喂给 Bevy 的。

相关文档：

- 资源目录说明：`assets/README.md`
- AI 协议边界：`AI_PROTOCOL/ASSETS.md`
- 第三方来源/许可证记录：`docs/assets.md`
- 3D 表现层代码：`crates/render_3d/README.md`

本文只讲“资源是什么”，不讲“代码怎么写”。

---

## 1. 先建立一个心智模型

3D 游戏画面里你看到的每一样东西，本质上都是下面这几层叠出来的：

```text
形状(mesh)  +  表面外观(material + texture)  +  骨架(skeleton/rig)  +  动作(animation)
                                |
                                v
                        放进场景(scene) + 打上灯光(lighting)
                                |
                                v
                        相机拍下来 -> 屏幕上的画面
```

把它和现实类比一下，会非常好理解：

| 现实世界 | 3D 游戏里的资源 | 目录 |
|---|---|---|
| 雕塑的形状 | mesh（网格） | `models/` |
| 雕塑表面刷的漆、贴的花纹 | texture（贴图） | `materials/{name}/` |
| “这个表面是金属还是橡胶”的规则 | material（材质） | `materials/{name}/` |
| 人体内的骨头 | skeleton（骨架） | `skeletons/` |
| 骨头和肌肉皮肤的绑定关系 | rig（绑定） | `rigs/` |
| 一段舞蹈动作 | animation（动画） | `animations/` |
| 整个房间的布置 | scene（场景） | `scenes/` |
| 天空、阳光、环境光 | environment-map / lightmap / irradiance-volume | 对应同名目录 |
| 烟、雾这种没有固定形状的东西 | volume（体积） | `volumes/` |

**关键认知**：这些大多数时候不是分开的一堆文件，而是被打包进**一个 `.glb` 文件**里。后面会详细讲。

---

## 2. 一个 3D 物体最少需要什么

如果只想让一个物体出现在屏幕上，最少需要两样东西：

1. **mesh（网格）** —— 它的形状
2. **material（材质）** —— 它的表面长什么样

### 2.1 Mesh（网格）— 形状

Mesh 是 3D 物体的“骨肉形状”，由大量**顶点(vertex)**连成的**三角形(triangle)**拼出来的壳。

```text
一个立方体 = 8 个顶点，12 个三角形面
一个游戏角色 = 几千到几万个三角形
```

你不需要手写顶点。Mesh 是美术在 **Blender / Maya / 3ds Max** 这类建模软件里做出来的，导出时一起带走。

> 在本项目里，mesh 几乎总是包含在 `models/` 里的 `.glb` 文件里，不会单独存一个“纯 mesh 文件”。

### 2.2 Material（材质）— 表面的“物理规则”

同样一个球的形状，可以是：

- 一个生锈的铁球
- 一个光滑的玻璃球
- 一个粗糙的木球

形状(mesh)一样，差别全在 **material**。

现代 3D 游戏用 **PBR（Physically Based Rendering，基于物理的渲染）** 来描述材质。PBR 的意思是：用几个符合物理直觉的参数来描述一个表面，光照系统就能算出它在任何光线下应该长什么样。

PBR 的核心参数（你会反复看到这些词）：

| 参数 | 含义 | 直觉 |
|---|---|---|
| base color / albedo | 基础颜色 | 不考虑光照时，物体“本身”的颜色 |
| metallic | 金属度 | 0 = 非金属（木头/塑料），1 = 金属 |
| roughness | 粗糙度 | 0 = 镜面般光滑，1 = 磨砂般粗糙 |
| normal | 法线 | 让平面看起来有凹凸细节（假的凹凸，省三角形） |
| ao（ambient occlusion） | 环境光遮蔽 | 缝隙、角落里更暗的效果 |
| emissive | 自发光 | 物体自己发光，比如霓虹灯、岩浆 |

这些参数可以是**一个固定数值**（整个物体都一样），也可以是**一张贴图**（表面每个点不一样）—— 这就引出了下一个概念。

### 2.3 Texture（贴图）— 表面的“图案”

Texture 就是普通的图片文件（`.png` / `.ktx2` 等），贴在 mesh 表面，给每个点提供细节。

PBR 材质的每个参数都可以对应一张贴图：

```text
albedo.png      -> 每个点的颜色（比如砖墙的红白花纹）
normal.png      -> 每个点的凹凸（比如砖缝的深浅，蓝紫色的图）
roughness.png   -> 每个点的粗糙度（比如磨损区域更粗糙）
metallic.png    -> 每个点是不是金属
ao.png          -> 每个点的阴影遮蔽
emissive.png    -> 每个点是否发光
```

> **base-color / albedo 是同一张贴图的两种叫法**，都指“基础颜色贴图”。

把这几张贴图叠在一起，一面普通的墙就能看起来像真实的砖墙，而 mesh 本身可能只是一个平板（normal 贴图负责伪造凹凸感）。

这就是为什么一个材质目录里会看到 `albedo.png`、`normal.png`、`roughness.png` 这一组文件——它们是同一个材质的不同“图层”。在本项目里，普通材质贴图跟着材质放到 `assets/3d/materials/{material-name}/`，而不是单独放一个顶层 `textures/`。

---

## 3. 让物体动起来：骨架、绑定、动画

静态的石头、桌子、墙，有 mesh + material 就够了。

但角色、怪物、会开合的门，需要**动**。这需要三个新概念：

### 3.1 Skeleton（骨架）

在 mesh 内部放一套虚拟的“骨头”（专业叫 **bone / joint**），像人体骨架一样。骨头本身不可见，它只是一组可以旋转移动的控制点。

```text
角色骨架（简化）：
  根 -> 脊椎 -> 脖子 -> 头
             -> 左肩 -> 左肘 -> 左手
             -> 右肩 -> 右肘 -> 右手
             -> 左髋 -> 左膝 -> 左脚
             -> 右髋 -> 右膝 -> 右脚
```

对应目录：`skeletons/`。

### 3.2 Rig（绑定 / 蒙皮）

Rig 解决一个问题：**骨头动的时候，皮肤（mesh）要跟着怎么动？**

绑定（rigging / skinning）就是告诉引擎“每根骨头控制 mesh 上哪些顶点、影响多大”。绑好之后，抬一下手臂骨头，对应的皮肤网格就会跟着弯曲。

Rig 还包括“控制器”和**重定向(retargeting)**信息——比如让一套通用人形动画能套用到不同体型的角色上。

对应目录：`rigs/`。

### 3.3 Animation（动画）

Animation 是一段**随时间变化的骨头姿势记录**，专业叫 **keyframe（关键帧）** 序列。

```text
“走路”动画 = 第0帧的姿势 -> 第10帧的姿势 -> ... -> 循环
“攻击”动画 = 一段挥剑的姿势序列
“死亡”动画 = 倒下的姿势序列
```

一个角色通常有一**组**动画：idle（待机）、walk、run、jump、attack、hit、die…… 游戏代码根据状态切换播放哪一段。

对应目录：`animations/`。

> **骨骼动画 vs 程序动画**：上面讲的是“骨骼动画”（事先做好的）。还有一种是代码实时算出来的（比如让方块匀速旋转），那种不需要 asset，直接在 `render_3d` 里写逻辑，不属于 `assets/3d`。

---

## 4. 关键：`.glb` 把上面这些打包在一起

这是 3D 新手最容易困惑的点，单独讲清楚。

前面把 mesh、material、texture、skeleton、rig、animation 拆开讲，是为了让你理解**概念**。但实际拿到的文件里，它们通常是**打包在一起的**。

**glTF / GLB 是 3D 界的“通用交换格式”**，地位类似 2D 界的 PNG、文档界的 PDF。

- `.gltf` —— 文本格式，贴图等资源是分开的外部文件
- `.glb` —— 二进制格式，**把 mesh + material + texture + skeleton + animation 全部塞进一个文件**

一个角色的 `character.glb` 里可能已经包含了：形状、所有贴图、骨架、绑定、以及 idle/walk/run 等所有动画。

当前模板的 runtime 3D 模型优先使用 `.glb`。

这就对应了 AI 协议里的资源边界：

> 贴图、材质、骨骼和动画如果已经打包进 `.glb`，不需要额外拆出来。只有需要复用、替换或单独管理时才拆到对应目录。

也就是说：

```text
拿到一个 character.glb（自带贴图和动画）
  -> 直接放 assets/3d/models/character/character.glb 就行
  -> 不需要去 materials/ animations/ 里再拆一份

只有当你想：
  - 多个模型共用同一套贴图
  - 给一个 mesh 换不同材质
  - 把一套动画套到多个角色上
才需要把对应资源拆出来，单独放进 materials/ animations/
```

新手阶段：**绝大多数情况，一个 `.glb` 就是一个完整可用的 3D 物体**。

---

## 5. 让世界“好看”：环境与光照

有了物体，还要有“世界的氛围”。这部分资源决定画面是白天还是黄昏、是室内还是户外。

### 5.1 Scene（场景）

把多个模型、灯光、相机位置组合成一个“整体场景”——比如一整个房间、一片森林、一关地图。可以是一个大的 `.glb`，也可以是项目自定义的场景数据。

对应目录：`scenes/`（注意 `assets/3d/scenes/` 是 3D 场景模型；`assets/scenes/` 是 runtime 场景数据，两者不同）。

### 5.2 Environment Map（环境贴图）

环境贴图描述“物体周围的环境长什么样”，主要有两个用途：

1. **天空盒(skybox)**：玩家看到的天空/远景背景。
2. **反射与环境光**：金属球面上能反射出周围环境，靠的就是它。

常见形式是 **HDRI**（高动态范围全景图），一张 360° 的环境照片。

对应目录：`environment-maps/`。

### 5.3 Lightmap（光照贴图）

实时计算光照很费性能。对于不动的物体（墙、地面、建筑），可以**事先把光影“烤”成一张贴图**贴上去，运行时直接用，这叫 **baked lighting（烘焙光照）**。

对应目录：`lightmaps/`。

### 5.4 Irradiance Volume（辐照体积 / 光照探针）

用来给**会动的物体**提供“它当前位置应该被多少环境光照亮”的信息。本质是空间里布满一堆采样点（probe），记录每个点的环境光，让移动的角色走到阴影里会自然变暗、走到亮处会变亮。

对应目录：`irradiance-volumes/`。

> 5.3 和 5.4 都属于**全局光照(GI)** 进阶话题。新手可以先跳过，用引擎默认的实时光照就能跑。

### 5.5 Volume（体积）

没有固定表面、需要填充一个空间的效果：雾、烟、云、体积光。数据形式通常是 3D 纹理（voxel）。

对应目录：`volumes/`。

---

## 6. 把它串起来：从 0 到一个 3D 画面

按依赖顺序，一个 3D 物体出现在屏幕上的完整链条：

```text
1. 美术在 Blender 里建 mesh（形状）
2. 给 mesh 画 / 指定 PBR 材质和贴图（外观）
3. 如果要动：加骨架 -> 绑定 -> K 动画
4. 导出成 character.glb（以上全部打包）
        |  源文件 .blend 留在 workbench/，不进 assets/
        v
5. 把 character.glb 放进 assets/3d/models/character/
6. render_3d 代码用 AssetServer 加载它
7. 放进 scene，加灯光，加相机
        v
   屏幕上出现一个可以动的 3D 角色
```

**做一个最小可玩 3D 游戏，原始资源清单大概是：**

| 必需 | 说明 |
|---|---|
| ✅ 角色 / 主体模型（`.glb`） | 至少一个能动的主角，自带 idle/walk 动画 |
| ✅ 环境 / 关卡模型（`.glb`） | 地面、墙、障碍——可以静态 |
| ✅ 一张环境贴图(HDRI) | 天空 + 基础环境光，画面立刻不再“悬浮在虚空” |
| 🔸 音频、UI、字体 | 不在 `assets/3d`，但游戏需要（见 `assets/README.md`） |
| ⬜ lightmap / irradiance / volume | 进阶画质，初版可全部不要 |

灯光和相机**不是 asset**，是在 `render_3d` 代码里创建的，所以不在这张表里。

---

## 7. 本项目的资源组织方式

这一节解释为什么 `assets/3d/` 是**按资源类型分目录**，而不是按角色/道具/敌人分。

### 7.1 按“资源类型”分，不按“游戏对象”分

`assets/3d` 顶层目录是 `models / materials / animations / ...`，**不会**出现 `player/`、`enemy/`、`sword/` 这种业务分类。

原因：

> 业务对象如何组合 model、material、animation、rig，属于代码、`render_3d` 配置或 prefab 层，不属于 `assets/3d` 的目录职责。

也就是说——“玩家由哪个模型 + 哪套动画 + 哪个材质组成”这件事，是**代码/prefab**决定的，不是靠目录结构表达的。资源目录只负责按类型把原料码放整齐。

### 7.2 源文件不进 `assets/`

```text
assets/  ->  只放 runtime 直接加载的成品（主要是 .glb 和贴图）
workbench/  ->  放 .blend / .fbx / .obj 源文件、参考图、离线工具输入输出
```

`.blend`、`.fbx`、`.obj` 这些**源文件 / 中间交换格式通常放在 `workbench/`**，`assets/` 只放 runtime 直接加载的成品。runtime 代码也只加载 `assets/`。

### 7.3 命名与稳定性

- 文件和目录用 **lowercase kebab-case**（小写 + 连字符）。
- 路径保持稳定，减少无意义重命名——因为 Bevy 的 asset handle 和代码里写死的路径都依赖它。

```rust
// 代码里这样引用，路径相对 assets/
asset_server.load("3d/models/character/character.glb");
```

### 7.4 第三方资源要登记

从网上下载的免费/付费模型，建议把**来源 URL、许可证、用途**记录到 `docs/assets.md`。3D 资源的授权比较容易踩坑（很多“免费”模型禁止商用），这一步很值得保留。

### 7.5 当前模板状态

- 模板**不携带默认 3D 资源**，`assets/3d/` 目录是空的占位结构。
- `render_3d` 子包存在，但**默认 app 跑的是 2D**（`Render2dPlugin`）。
- 真要做 3D 时，才在顶层组装 `Render3dPlugin`。

所以现在 `assets/3d/` 里没东西是**正常的**——它是给你预留好的、规范的“货架”，等你往里放料。

---

## 8. 名词速查表

| 名词 | 一句话解释 |
|---|---|
| mesh | 由三角形拼成的 3D 形状 |
| vertex | mesh 的顶点 |
| material | 表面的“物理规则”（金属/粗糙/发光…） |
| PBR | 基于物理的渲染，用统一参数描述材质 |
| texture | 贴在表面的图片，提供细节 |
| albedo / base-color | 基础颜色贴图（同一个东西） |
| normal map | 伪造表面凹凸的贴图 |
| roughness / metallic | 粗糙度 / 金属度 |
| ao | 环境光遮蔽（缝隙变暗） |
| emissive | 自发光 |
| skeleton / bone | 角色内部的虚拟骨架 |
| rig / skinning | 把骨头和皮肤网格绑定起来 |
| animation / keyframe | 随时间变化的骨头姿势序列 |
| glTF / GLB | 3D 通用交换格式，`.glb` 把一切打包进单文件 |
| scene | 多个物体 + 灯光 + 相机的组合 |
| HDRI / environment map | 360° 环境/天空，提供背景和反射 |
| lightmap | 预先烤好的静态光影贴图 |
| irradiance volume / probe | 给动态物体提供环境光的采样点 |
| volume | 雾/烟/云这类填充空间的体积效果 |
| baked / GI | 烘焙 / 全局光照（进阶画质） |

---

## 9. 给完全的新手：最小起步路线

如果你想先跑通，不追求画质，建议顺序：

1. 去模型站（如 Kenney、Quaternius、Sketchfab 上的 CC0 资源）下一个 **自带动画的 `.glb` 角色**。
2. 登记来源到 `docs/assets.md`，放进 `assets/3d/models/`。
3. 找一张 HDRI 放进 `assets/3d/environment-maps/`。
4. 在 `render_3d` 里接入 `Render3dPlugin`，加载模型、加相机和一盏默认灯。
5. 能看到角色站在天空背景里、动起来——**到这一步你就跑通了整条 3D 资源链路**。

之后再逐步加：关卡模型、更多动画、lightmap、volume 特效……每一样都对应 `assets/3d/` 里一个已经给你建好的目录。

记住一句话收尾：

> 3D 资源不神秘——**形状 + 表面 + 骨架 + 动作，打包成 `.glb`，放进对应类型的目录，代码按路径加载。** 其余都是在这之上叠画质。
