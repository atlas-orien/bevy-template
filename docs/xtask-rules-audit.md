# xtask 规则审计

本文档记录当前 `crates/xtask/src/rules/` 的规则状态。

## 已删除的规则类型

以下规则已经从有效检查、调用点和过渡 no-op 函数中移除：

- 具体文件必须存在。
- 具体函数名、类型名必须出现在某文件。
- 具体 demo 文件必须存在。
- 具体 demo 资源路径必须由某 crate 管理。
- 具体目录只允许某些文件。
- 具体目录只允许某些子目录。
- 固定依赖版本。
- Cargo manifest 依赖白名单/黑名单。
- 强制所有 crate 依赖 `error`。
- 禁止标准 `Result` 或其它 crate 定义 `Result` alias。
- crate-level `//!` 文档强制检查。
- Rust 文件 400 行限制。
- `assets/2d/animated` 下 png 必须有同名 `.frames.ron`。
- render 文件只允许一个 public `Bundle` 产品入口。
- prefab 禁止所有 `.insert(`。

说明：

- 文件长度规则没有继续保留为有效检查。之前讨论过 1000 行阈值，但最终按“容易误伤的风格规则删除”处理。
- `require_path`、`require_paths`、`require_file_contains_all_terms`、`reject_files_under_dir_except`、`reject_subdirs_except` 已删除。
- crate 规则构造参数中的 `required_paths`、`allowed_files`、`required_api_terms`、`content_dirs`、`forbidden_dependencies` 等路径白名单/依赖清单字段已删除。
- `base/assets.rs`、`base/readability.rs`、`base/render_api.rs`、`base/dependencies.rs` 已从规则模块中删除。

## 当前仍保留的规则类型

### 明确拒绝旧路径或错误路径

保留 `reject_paths`：

- 用来禁止已经明确废弃的旧目录。
- 例如旧的 flat render_2d 目录、旧 physics backend 目录、prefab/ui/camera.rs 等。

### AST 检查：散装 prefab tuple

保留：

- `.spawn((...))`
- `.insert((...))`

目的：

- 阻止 prefab 现场拼 loose tuple。
- 允许 `.insert(named_bundle_or_component)`，因为全量禁止 `.insert(` 容易误伤合法窄桥接。
- 实现方式是 AST method call 检查，只匹配单参数 tuple 调用，不扫描源码字符串。

### AST 检查：资源路径写错层

保留 prefab 中的资源路径禁止项：

- `"audio/"`
- `"2d/"`
- `"3d/"`
- `.ogg`
- `.wav`
- `.mp3`
- `.png`
- `.jpg`
- `.jpeg`
- `.ron`
- `.glb`
- `.gltf`

目的：

- prefab 不硬编码具体资源路径。
- 资源路径由 catalog 或更上层资源绑定处理。
- 实现方式是检查 Rust 字符串字面量，不扫描注释或普通源码文本。

### AST 检查：直接输入源

保留对以下类型/路径的检查：

- `ButtonInput`
- `KeyCode`
- `MouseButton`
- `Gamepad`

用于阻止不该读输入的 crate 直接读键盘、鼠标、手柄。

### AST 检查：Bevy World / world mutation

保留对以下类型/路径的检查：

- `World`
- `Commands`
- `Query`
- `Res`
- `ResMut`
- `Transform`
- `PhysicsBody`
- `PhysicsCollider`

用于阻止 external runtime、intent、peripherals、interaction 等边界层直接改 Bevy World。

### AST 检查：网络传输出现在错误层

保留对以下类型/路径的检查：

- `protobuf`
- `prost`
- `socket`
- `TcpStream`
- `UdpSocket`
- `WebSocket`

用于阻止 peripherals、interaction 等层直接处理网络传输。

### AST 检查：ECS 数据定义出现在错误层

保留 derive 检查：

- `Component`
- `Bundle`
- `Resource`
- `Event`
- `Message`

用于确保 ECS 数据、消息和资源放在约定层。

### AST 检查：render/gameplay 反向语义

保留一些类型/路径语义检查，例如：

- render_2d 不写 `PhysicsCollider`、`Hitbox`、`Hurtbox`、`Combo`、`SkillWindow` 等玩法概念。
- physics 不写 `Hitbox`、`Hurtbox`、`AttackRange`、`SkillRange` 等 gameplay 语义。
- network 不写 `RuntimeRequestMessage`、`RuntimeUserId`、`RuntimeObjectId`。

这些检查基于 Rust AST 的 use tree / path visitor，不再扫描源码字符串。

### AST 检查：硬编码 atlas 切图 API

保留对以下路径后缀的检查：

- `TextureAtlasLayout::from_grid`
- `ImageArrayLayout::RowCount`
- `ImageArrayLayout::ColumnCount`

目的：

- frame animation 的 sheet layout、clips、fps 和 repeat 数据应来自 `.frames.ron` manifest。
- 检查基于 AST path suffix，不会误伤合法的 `TextureAtlasLayout` handle 类型。

### AST 检查：free function API

保留 UI products 中的 free function 检查：

- free function 返回 `impl Bundle`
- free function 返回 `Node`

目的：

- UI 表现应使用命名 bundle/component，而不是散落 helper 函数。
- 实现方式是检查函数返回类型 AST，不扫描函数签名字符串。

### Rust 模块完整性检查

保留：

- 目录模块需要 `mod.rs`。
- 禁止目录内出现和父目录同名的 `.rs` 文件。

这些属于 Rust 模块结构完整性，不是业务目录白名单。
