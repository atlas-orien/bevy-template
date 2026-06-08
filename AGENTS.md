# AI 入口

本文件写给 AI 代理。

项目的详细 AI 协议在根目录 `AI_PROTOCOL/`。

开始任何代码修改前，先读取并遵守相关的 `AI_PROTOCOL/*.md` 文件。

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
