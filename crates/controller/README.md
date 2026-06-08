# controller

`controller` 是控制来源到意图数据的转换层。

它负责把键盘、手柄、AI、脚本、网络等控制来源转换成 ECS intent 组件。它不决定世界结果，只表达“想做什么”。

## 职责

- 读取控制来源。
- 写入 ECS intent 组件。
- 根据控制来源更新朝向、目标、使用物品意图等轻量控制数据。
- 保持不同控制来源分离。

## 推荐结构

- `keyboard`: 键盘和鼠标输入。
- `gamepad`: 手柄输入。
- `ai`: AI 决策到 intent。
- `script`: 脚本、剧情、触发器到 intent。
- `network`: 网络消息到 intent。

## 和 simulation 的区别

`controller` 只表达“想做什么”，`simulation` 决定这些 intent 在什么状态和阶段被调度。

例如：

- 玩家想向左移动。
- AI 想攻击目标。
- 脚本想打开门。

真正修改 `Transform`、生命值、背包、世界状态的系统函数应该放到 `crates/ecs/src/systems`。

如果这些系统需要按游戏状态、关卡阶段或流程组合，再由 `simulation` 负责调度。

## 和 prefab 的区别

`controller` 不生成实体，也不关心一个实体由哪些组件组合出来。

`prefab` 定义“生成什么组合”，`controller` 只写已有实体上的 intent。

## 不应该放这里

- 不直接生成实体。
- 不使用 prefab。
- 不直接修改 `Transform`、生命值、背包等世界结果。
- 不直接播放动画。
- 不直接修改渲染组件。
- 不封装物理后端。
