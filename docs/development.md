# 开发说明

这个目录用于记录功能简报、实现路线和架构决策。

当前模板按分层 crate 组织：

- `error`：共享错误类型、错误报告和统一 `Result`。
- `ecs`：核心 ECS component、resource、event/message 和 system。
- `external_runtime`：Bevy App 外部 runtime、外部来源轮询和 manager-side adapter。
- `intent`：实体意图写入 API。
- `gameplay`：游戏状态流、阶段调度和高层 ECS system 编排。
- `render_2d`：2D 渲染和表现层。
- `render_3d`：3D 渲染和表现层。
- `app`：最终 Bevy App 组装 crate。

规划新功能时，先决定每一部分属于哪一层，再写代码。

可失败的项目函数应返回 `error::Result<T>`，并用 `error::GameError` 构造错误。

建议的功能简报格式：

```md
# 功能名称

## 目标

## 玩家体验

## 技术说明

## 验收检查
```
