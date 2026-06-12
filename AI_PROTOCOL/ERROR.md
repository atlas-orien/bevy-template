此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# ERROR

这个文件是 `crates/error` 的 AI 规则。

`crates/error` 是全项目唯一错误层。

## 统一 Result

所有 crate 的可失败项目函数都必须使用：

```rust
error::Result<T>
```

非 `error` crate 可以重新导出：

```rust
pub use error::Result;
```

但不允许自己定义新的 `Result` 类型别名。

## 边界规则

- 只有 `crates/error` 可以定义 `Result<T>`。
- 只有 `crates/error` 可以直接使用 `core::result::Result` 或 `std::result::Result` 作为统一别名的底层实现。
- 其它 crate 不允许定义 `type Result<T> = ...`。
- 外部错误转换集中放在 `crates/error`。
- `GameError` 使用 `thiserror` 定义。
- `error` 不注册 Bevy plugin。
- `error` 不定义 Bevy ECS 数据、message 或 gameplay/runtime system。

## Cargo 规则

- 每个 crate 都必须依赖 `error`。
- 每个非 `error` crate 都应该通过 `pub use error::Result;` 复用统一 Result。

## 验证要求

修改错误处理相关代码后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
