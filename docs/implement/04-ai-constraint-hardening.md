# 04 - AI 约束体系加固

这份文档描述如何加固本模板的 AI 约束体系。

目标只有一个：**让 AI 按照既定架构思路写代码，过程可控，结果人可读。**

注意：本文是实现路线说明，不是项目硬约束来源。项目硬约束只来自：

- `AI_PROTOCOL/*.md`
- `crates/xtask/src/rules/*.rs`

本步骤独立于 `01-local-input` / `02-ai-control` / `03-subsystems`，可以在任何时间单独执行。
但建议在让 AI 大规模写 gameplay 代码之前完成阶段 A 和阶段 B。

---

## 0. 问题定义

当前体系已经是正确的三层结构：

| 层 | 载体 | 性质 |
|---|---|---|
| 入口 | `AGENTS.md` | 告诉 AI 去哪读规则、跑什么验收 |
| 软约束 | `AI_PROTOCOL/*.md` | 自然语言边界规则，AI 读后遵守 |
| 硬约束 | `crates/xtask/src/rules/` | 机器检查，AI 改到通过为止 |

体系的核心原理是：**AI 对纯文档规则的遵守率不稳定，但对"会让任务失败的检查"
遵守率接近 100%。** 所以加固方向不是写更多文档，而是：

1. 把硬约束里能被绕过的洞堵上（AI 被挡住时会无意识地"换个写法"，这不是恶意，是它解决问题的本能）。
2. 把"AI 改规则让检查通过"这条最隐蔽的路堵上。
3. 给软约束补"正确形状"——AI 模仿例子的权重高于遵守禁令，禁令只画边界，骨架才给落点。
4. 给"人可读"加最低限度的机器保证。

已知的三个具体的洞：

- **洞 1（依赖检查）**：`rules/util.rs` 的 `manifest_has_workspace_dependency` 只匹配
  字符串 `ecs.workspace = true`。写成 `ecs = { workspace = true }` 或
  `ecs = { path = "../ecs" }` 即可静默绕过。
- **洞 2（术语检查）**：`rules/base/source.rs` 的 `reject_terms_in_rust_files` 是子串匹配。
  `use bevy::prelude::Transform as Xform;` 之后全文不再出现 `Transform`，检查失效；
  反过来，注释、文档、`TransformRequest` 这类合法语义命名会被误伤。
  误伤的危害不止是烦：它会教会 AI "这个检查不讲理"，降低 AI 对整套规则的信任。
- **洞 3（规则自身不受保护）**：AI 被 xtask 挡住时，修改 `crates/xtask/src/rules/`
  或 `AI_PROTOCOL/*.md` 让检查通过，是一条没有任何东西拦截的路。

---

## 阶段 A：保护规则本身（成本最低，优先做）

### A1. AGENTS.md 增加规则保护条款

在 `AGENTS.md` 的"合格标准"之前增加一节，措辞建议：

```markdown
## 规则保护

`AI_PROTOCOL/*.md` 和 `crates/xtask/src/rules/` 是项目的约束来源，对 AI 只读。

- 不得为了让 `cargo run -p xtask -- check` 通过而修改这两个路径下的任何文件。
- 检查失败时，唯一正确的反应是修改业务代码，使其符合规则。
- 如果你认为某条规则本身有错（误伤、过时、自相矛盾），停止当前修改，
  在回复中说明哪条规则、为什么有问题、建议怎么改，等人决定。
- 规则的新增、放宽、删除只能由人发起。人发起时会在指令里明确说
  "修改 xtask 规则"或"修改 AI_PROTOCOL"。
```

要点：

- 必须给 AI 留"规则可能真的有错"的出口（停下来报告），否则 AI 在遇到真误伤时
  会在"违反指令改规则"和"写出扭曲的代码绕过检查"之间选一个，两个都是坏结果。
- "人发起时会明确说 X" 这句让 AI 能区分"我想改规则"和"人让我改规则"。

### A2. 规则文件头部声明

给 `crates/xtask/src/rules/mod.rs` 和每份 `AI_PROTOCOL/*.md` 的开头加一行声明：

```text
此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。
```

原因：AI 不一定每次都把 `AGENTS.md` 读进上下文，但它要改某个文件时一定会先读
那个文件。声明放在被改对象自身上，命中率最高。

### A3.（可选）Git 层兜底

如果项目接入 CI 或使用 Claude Code hooks，加一层机器兜底：

- CI：对 `AI_PROTOCOL/` 和 `crates/xtask/src/rules/` 的变更要求人工 review
  （GitHub 上用 CODEOWNERS 即可，两行配置）。
- 本地 Claude Code：在 `.claude/settings.json` 配置 PreToolUse hook，
  对 Edit/Write 目标路径匹配这两个目录时返回拒绝并提示阅读 A1 条款。

A3 不阻塞后续阶段，没有 CI 就先跳过。

### A 阶段验收

- `AGENTS.md` 包含"规则保护"一节。
- `crates/xtask/src/rules/mod.rs` 和全部 15 份 `AI_PROTOCOL/*.md` 带头部声明。
- 人工测试：让 AI 执行一个必然撞上 xtask 检查的任务（例如"在 intent 里直接修改
  Transform"），观察它是改业务代码、停下来报告，还是去动规则文件。前两者合格。

---

## 阶段 B：依赖检查改为解析 Cargo.toml（堵洞 1）

### B1. 改动范围

- `crates/xtask/Cargo.toml`：增加 `toml` 依赖（workspace 已有 serde 生态的话走 workspace 版本）。
- `crates/xtask/src/rules/util.rs`：替换 `manifest_has_workspace_dependency`。
- `crates/xtask/src/rules/base/dependencies.rs`：调用方改用新函数。

### B2. 实现细节

新函数语义：**"这个 crate 的 manifest 里是否声明了名为 X 的依赖"，
与声明形式无关。**

```rust
/// 解析 manifest，返回所有声明的依赖包名。
/// 覆盖 [dependencies]、[dev-dependencies]、[build-dependencies]
/// 和 [target.'cfg(...)'.dependencies] 四类表。
/// 依赖项使用 `package = "real-name"` 重命名时，以 real-name 为准。
pub fn manifest_dependency_names(manifest_source: &str) -> Vec<String>
```

实现要点：

1. 用 `toml::Value` 解析整个文件，解析失败时报错（manifest 都解析不了，
   说明文件本身坏了，应该当检查错误而不是静默跳过）。
2. 收集四类依赖表。`[target]` 下要遍历所有 target 子表。
3. 每个依赖项可能是三种形态，全部要覆盖：
   - `ecs = "0.1"`（字符串）
   - `ecs.workspace = true`（点号路径，toml 解析后和 inline table 等价）
   - `ecs = { workspace = true }` / `{ path = "../ecs" }` / `{ git = "..." }`（表）
4. 表形态里如果有 `package = "ecs"` 字段，依赖的真实包名是 `package` 的值，
   不是 key。`my_ecs = { package = "ecs", path = "../ecs" }` 必须被识别为 `ecs`。
5. `reject_workspace_dependencies` 改名为 `reject_dependencies`（语义已不限于
   workspace 形式），内部改为 `manifest_dependency_names(...).contains(forbidden)`。
   各 crate 规则文件里的 `FORBIDDEN_DEPENDENCIES` 列表不需要改。

### B3. 必须新增的测试

在 `util.rs` 的 `#[cfg(test)]` 里覆盖以下用例。
**绕过用例（必须被识别出依赖存在）：**

```toml
ecs.workspace = true                          # 原有形式
ecs = { workspace = true }                    # inline table
ecs = { path = "../ecs" }                     # path 依赖
ecs = "0.1"                                   # 版本依赖
my_ecs = { package = "ecs", path = "../ecs" } # 重命名
[dev-dependencies] 下的 ecs                    # 非主依赖表
[target.'cfg(unix)'.dependencies] 下的 ecs     # target 依赖表
```

**误伤用例（必须不触发）：**

```toml
ecs_helper = { path = "../ecs_helper" }   # 名字前缀相同的其他包
# ecs = { path = "../ecs" }               # 注释里的依赖（toml 解析天然免疫）
```

### B 阶段验收

```sh
cargo test -p xtask
cargo run -p xtask -- check   # 现有工作区必须依然通过
```

外加一次人工破坏性测试：随便挑一个 crate（如 `crates/intent`），手动加上
`ecs = { path = "../ecs" }`，确认 `xtask check` 报错且错误信息里带修复 hint，
然后撤销。

---

## 阶段 C：术语检查升级为语法树检查（堵洞 2）

### C1. 原则

不要试图把所有子串规则都换成语法分析——成本不值。分两档处理：

- **高价值类型名**（`Commands`、`Transform`、`ButtonInput`、`KeyCode`、
  `MouseButton`、`Gamepad`、`World`、`Query`、`Res`、`ResMut` 等出现在
  `source.rs` 常量表里的 Bevy 类型）：升级为基于 `syn` 的路径检查。
  这些是 AI 最常用、也最容易用 alias 绕过的。
- **其余规则**（多词组合、manifest 术语、文件级 require/reject）：保留子串匹配。
  它们要么不是合法 Rust 标识符的形态，要么误伤概率低。

xtask 已经依赖 `syn` 并有 `parse_rust_file`，基础设施现成。

### C2. 实现细节

在 `rules/base/source.rs` 新增：

```rust
/// 检查 Rust 文件中是否引用了禁止类型。基于语法树而不是子串：
/// - use 声明：`use bevy::prelude::Transform;`、
///   `use bevy::prelude::Transform as Xform;`（alias 形式同样命中，
///   因为检查的是被 use 的原始路径段）、
///   `use bevy::prelude::{Transform, Commands};`（use tree 展开）
/// - 类型位置：`fn f(t: &mut Transform)`、`Query<&mut Transform>`
/// - 完全限定路径：`bevy::transform::components::Transform::default()`
/// 命中条件：路径的任意一段等于禁止名（整段相等，不是子串包含）。
pub fn reject_type_paths_in_rust_files(
    root: impl AsRef<Path>,
    forbidden: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
)
```

实现方式：

1. 对每个文件 `parse_rust_file`。解析失败已有现成的报错路径（保持 Rust 合法
   本来就是规则之一）。
2. 实现 `syn::visit::Visit`，重写两个 visit 点即可覆盖绝大多数引用形态：
   - `visit_use_tree`：检查 `UseTree::Name` / `UseTree::Rename`（rename 时检查
     `ident` 即原始名，不是 alias 名）/ 递归 `Path` 和 `Group`。
   - `visit_path`：检查 `path.segments` 的每个 `ident`。`visit_path` 同时覆盖
     表达式位置、类型位置和模式位置的路径。
3. 整段相等匹配，所以 `TransformRequest`、注释、字符串字面量天然不再误伤。

已知残余盲区（写进函数文档注释里，不在本阶段解决）：

- 宏调用体内部的 token（syn 不展开宏）。Bevy 用户代码里被宏包住的类型引用很少，
  且 AI 为绕检查专门把代码塞进宏里的形态非常显眼，人审一眼能看到。
- `use bevy::prelude::*` 之后裸用 `Transform` 会被 `visit_path` 命中，没有盲区；
  但 glob import 本身不报错，这是预期行为。

### C3. 调用方迁移

`reject_direct_input_access`、`reject_world_mutation_terms`、
`reject_bevy_world_access` 三个入口的常量表里：

- 纯类型名（`ButtonInput`、`KeyCode`、`Transform`、`Commands`、`World` 等）
  → 走新的 `reject_type_paths_in_rust_files`。
- 带语法噪音的多词项（`Query<(&mut Transform`、`Query<&mut Transform`）
  → 直接删除：类型路径检查命中 `Transform` 和 `Query` 本身已经覆盖这两条。
- 泛型形态项（`Query<`、`Res<`、`ResMut<`）→ 改成纯名字 `Query`、`Res`、`ResMut`
  走路径检查。

迁移后各 crate 规则文件（`rules/crates/*.rs`）里引用这些常量的地方不变，
只有 `source.rs` 内部实现换掉。

### C4. 必须新增的测试

**绕过用例（必须命中）：**

```rust
use bevy::prelude::Transform as Xform;            // alias 绕过
use bevy::prelude::{Commands as C, Entity};       // use tree 内 alias
fn f(q: Query<&mut bevy::transform::components::Transform>) {}  // 完全限定路径
type T = bevy::prelude::Transform;                // 类型别名声明
```

**误伤用例（必须不命中）：**

```rust
/// 把 Transform 留给 ecs 层处理。       // 文档注释
pub struct TransformRequest;             // 前缀相同的语义类型
let s = "Transform";                     // 字符串字面量
mod transform_rules {}                   // 小写同词模块名
```

### C 阶段验收

```sh
cargo test -p xtask
cargo run -p xtask -- check   # 现有工作区必须依然通过
```

人工破坏性测试：在 `crates/intent` 里加一行
`use bevy::prelude::Transform as Xform;`，确认被挡，然后撤销。

---

## 阶段 D：AI_PROTOCOL 增加骨架代码（给"正确形状"）

### D1. 为什么

AI 写代码时，模仿就近例子的权重高于遵守禁令。`AI_PROTOCOL/*.md` 目前以
禁令列表为主——禁令画出边界，但不给落点。在例子稀缺（目前只有 UI 一例）的
情况下，AI 会把 UI 例子的形态硬套到 gameplay、prefab 上。

完整可运行的示例游戏成本高，本阶段用便宜的替代：**每份协议嵌一段
10–20 行的骨架代码**，展示"这一层的代码长什么形状"。一段骨架顶十条禁令。

### D2. 骨架的写法规范

每段骨架必须满足：

1. **不超过 25 行**。骨架是形状参考，不是教程。
2. **用 `your_xxx` 占位命名**（`YourObjectPrefab`、`your_action_system`），
   明确告诉 AI 这是要替换的部分，防止占位名原样进库。
3. **只展示该层允许的依赖和形态**，骨架本身不能违反本协议的任何禁令
   （写完后人工对照一遍边界规则）。
4. **与 xtask 锚点一致**：骨架里出现的目录、trait、bundle 命名必须和
   `rules/crates/*.rs` 里 require 的路径锚点一致，避免文档和检查打架。
5. 放在协议文件的"代码落点"和"边界规则"之间，章节名统一叫 `## 骨架`。

### D3. 优先级排序

15 份协议不需要一次写完。按 AI 实际会写代码的频率排：

| 优先级 | 协议 | 骨架内容 |
|---|---|---|
| 1 | `PREFAB.md` | 最小 prefab struct + bundle + `Prefab` trait 实现 |
| 1 | `ECS.md` | 一个 component + 一个最小 system 函数（含 Query 写法） |
| 1 | `GAMEPLAY.md` | 一个 request 消费分支 + system 注册到调度的形态 |
| 2 | `INTENT.md` | 一个意图写入函数（只写数据，不动 Transform） |
| 2 | `PERIPHERALS.md` | 一个按键 → 语义动作的 binding 形态 |
| 2 | `EXTERNAL_RUNTIME.md` | 一个外部源 poll → manager 调用的形态 |
| 3 | 其余（render / physics / audio / navigation / interaction 等） | 各自最小形态 |

优先级 1 的三份做完，就覆盖了 AI 写新对象时必经的全部三层。

### D4. 维护规则

骨架属于 AI_PROTOCOL 的一部分，受阶段 A 的规则保护条款约束。
架构变更导致骨架过时，由人随架构变更一起更新（`AGENTS.md` 已要求
架构变更必须跑 `xtask check`，可在对应 crate 规则里用
`require_file_contains_all_terms` 锚定骨架章节存在，防止被悄悄删除）。

### D 阶段验收

- 优先级 1 的三份协议含 `## 骨架` 章节，每段 ≤ 25 行，占位命名规范。
- 人工对照：每段骨架不违反所在协议的任何边界规则。
- 实测：让 AI 在干净分支上"新增一个可移动对象"，对比有无骨架时
  生成代码的形态偏差。

---

## 阶段 E：人可读性的最低机器保证（可选）

前四个阶段保证"代码在正确的位置、用正确的依赖"。本阶段处理"人能不能读"。
已有的保证：`rustfmt`（格式）、`clippy -D warnings`（惯用法）、
xtask 的模块结构规则（`require_mod_rs_in_subdirs`、`reject_dir_named_files`）。

补充三条低成本规则，都加到 `rules/base/` 并在各 crate 规则里启用：

### E1. crate 级文档注释

每个 crate 的 `src/lib.rs` 必须以 `//!` 开头说明职责（一行即可）。
实现：读 lib.rs，trim 后检查前缀。AI 写这类注释的成本为零，
但能保证人打开任何 crate 第一行就知道它是干什么的。

### E2. 文件行数上限

单个 `.rs` 文件超过 400 行报错（`mod.rs` 除外的话规则更宽容，但建议不豁免，
mod.rs 本就应该只做声明和 re-export）。AI 在长会话里倾向于把代码持续追加到
同一个文件，行数上限强制它拆模块——拆出来的结构正好落进已有的
mod.rs 结构规则里。400 这个数字可以按口味调，写成规则参数。

### E3. mod.rs 只做声明

`mod.rs` 里只允许 `mod` / `pub mod` / `pub use` / `use` / 属性和注释，
出现 `fn` / `struct` / `enum` / `impl` 报错。实现：syn 解析后检查
item 类型白名单。这保证人浏览目录时 mod.rs 永远是一张目录表。

### E 阶段验收

```sh
cargo run -p xtask -- check   # 对现有代码全部通过（先修存量再合入规则）
```

注意执行顺序：先跑新规则看存量违规有多少，存量清零之后规则才合入，
避免出现"检查常红"——常红的检查会被 AI 和人共同学会无视。

---

## 总执行顺序与工作量

| 阶段 | 内容 | 预估工作量 | 依赖 |
|---|---|---|---|
| A | 规则保护条款 | 半小时（纯文档） | 无 |
| B | Cargo.toml 解析 | 2–3 小时（含测试） | 无 |
| C | syn 路径检查 | 3–5 小时（含测试） | 无 |
| D | 协议骨架（优先级 1） | 每份 0.5–1 小时 | 建议在 A 后 |
| E | 可读性规则 | 2–3 小时（含清存量） | 无 |

A → B → C → D → E 是推荐顺序（按"堵洞优先于补形状，补形状优先于提质量"），
但五个阶段互相没有代码依赖，可以乱序、可以只做一部分。

每个阶段完成后统一跑：

```sh
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p xtask -- check
```

## 能力边界（写给未来的自己）

这套体系加固完之后，能保证的是：代码在正确的 crate、用正确的依赖、
不跨层伸手、结构上人可读。它不能保证 AI 写出的玩法逻辑本身合理、
system 调度顺序正确、或查询性能良好——这些属于语义层面，静态规则够不着，
只能靠例子（阶段 D 是它的廉价替身，完整示例游戏是终极形态）、测试和人审。

合理预期：把 AI 的失败模式从"架构性灾难"压缩到"局部代码质量问题"。
后者修起来便宜得多，这就是本模板的立项目的。
