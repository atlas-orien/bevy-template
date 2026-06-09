# 错误处理子包

`error` 是全项目统一错误层。

## 职责

- 定义 `GameError`。
- 定义统一的 `Result<T>`。
- 定义错误类型和严重级别。
- 集中维护外部错误到 `GameError` 的转换。

## 统一 Result

所有项目代码里的可失败函数都应该使用：

```rust
error::Result<T>
```

其他子包会重新导出这个类型，因此子包内也可以使用本子包的：

```rust
Result<T>
```

不要在其他子包里再定义新的 `Result` 别名。

除 `crates/error` 自身外，也不要在项目函数里直接使用 `std::result::Result` 或 `core::result::Result`。需要返回错误时，统一使用 `error::Result<T>`。

## thiserror

`GameError` 使用 `thiserror` 定义。

如果将来接入新的外部库，例如资源解析、存档、网络、配置读取，对应错误转换应该集中加在这里。

## 不应该放这里

- 不写业务规则。
- 不写渲染逻辑。
- 不写输入系统。
- 不注册 Bevy plugin。
- 不定义 Bevy ECS 数据、message 或 gameplay/runtime system。
