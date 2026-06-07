# Development Notes

Use this folder for feature briefs and implementation decisions.

This template is split into layered crates:

- `error`: shared error reporting and collection.
- `ecs`: reusable gameplay/domain ECS.
- `gameplay`: rules, flow, spawning, and simulation systems.
- `render_2d`: 2D rendering and presentation.
- `render_3d`: 3D rendering and presentation.
- `app`: final assembly crate.

When planning a feature, decide which layer owns each part before writing code.
Fallible project functions should return `error::Result<T>` and construct errors with `error::GameError`.

Suggested brief format:

```md
# Feature Name

## Goal

## Player Experience

## Technical Notes

## Acceptance Checks
```
