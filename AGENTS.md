# AI 入口

本文件写给 AI 代理。

项目的详细 AI 协议在根目录 `AI_PROTOCOL/`。

开始任何代码修改前，先读取并遵守相关的 `AI_PROTOCOL/*.md` 文件。

## 规则保护

`AI_PROTOCOL/*.md` 和 `crates/xtask/src/rules/` 是项目的约束来源，对 AI 只读。

- 不得为了让 `cargo run -p xtask -- check` 通过而修改这两个路径下的任何文件。
- 检查失败时，唯一正确的反应是修改业务代码，使其符合规则。
- 如果你认为某条规则本身有错（误伤、过时、自相矛盾），停止当前修改，
  在回复中说明哪条规则、为什么有问题、建议怎么改，等人决定。
- 规则的新增、放宽、删除只能由人发起。人发起时会在指令里明确说
  "修改 xtask 规则"或"修改 AI_PROTOCOL"。

## 合格标准

代码修改完成后，必须运行并通过：

```sh
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

如果修改了架构、crate 边界或 AI 协议，还必须运行：

```sh
cargo run -p xtask -- check
```
