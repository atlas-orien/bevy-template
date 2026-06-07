# AI Development Guide

This repository is a Bevy game template designed for iterative AI-assisted development.
It is a Cargo workspace. Domain crates come first, and the final app crate assembles them.

## Project Shape

- `src/main.rs`: root launcher for `cargo run`; keep it tiny.
- `crates/error/`: shared error crate. Put error types, error events, severity levels, and error collection here.
- `crates/ecs/`: domain ECS crate. Put components, bundles, resources, and data categories such as characters, background, items, and world here.
- `crates/gameplay/`: gameplay crate. Put rules, state flow, spawning, interaction, combat, inventory, and simulation systems here.
- `crates/render_2d/`: 2D rendering crate. Put 2D cameras, screen presentation, HUD, sprites, tilemaps, and 2D-only visual glue here.
- `crates/render_3d/`: 3D rendering crate. Put 3D cameras, scenes, lighting, meshes, and 3D-only visual glue here.
- `crates/app/`: final runnable app crate. It configures Bevy plugins and assembles workspace crates.
- `assets/`: runtime assets loaded by Bevy. Keep source art or large raw files outside this directory unless the game needs them at runtime.
- `docs/`: design notes, feature specs, AI task briefs, and implementation decisions.
- `tests/`: integration tests and behavior-focused tests.
- `tools/`: local helper scripts for asset processing, builds, or checks.

## Development Rules

- Do not add a `game_` prefix to crate names; this repository is already a game template.
- This template is loaded from GitHub/workspace paths and is not published to crates.io. Keep packages marked `publish = false`.
- Route recoverable and fatal errors through `crates/error`; do not create isolated error systems in feature crates.
- Add external error conversions to `crates/error` with `thiserror`; do not scatter `From` conversions across feature crates.
- Project functions that can fail must return `error::Result<T>` unless they are implementing an external trait with a required signature.
- Every non-error workspace crate re-exports this as `crate::Result`; use that alias inside the crate.
- In Bevy 0.18, shared reporting uses Bevy messages. Emit `ErrorEvent` through the message system.
- Keep raw ECS/domain definitions in `crates/ecs`.
- Keep gameplay rules and systems in `crates/gameplay`.
- Keep 2D-specific rendering, screens, camera, and UI in `crates/render_2d`.
- Use `crates/render_3d` for 3D rendering; do not mix 3D concerns into `render_2d`.
- Keep `render_2d` and `render_3d` independent. The app crate chooses which renderer to assemble.
- Keep final app wiring in `crates/app`.
- Prefer Bevy plugins for feature boundaries.
- Prefer app states for screen-level flow.
- Keep `src/main.rs` tiny; it should call the app crate.
- Put reusable data in resources and reusable behavior in systems.
- Add marker components for entities that need cleanup on state exit.
- Do not mix UI spawning, gameplay simulation, and asset loading in one module.
- Rendering crates may depend on `ecs` and `gameplay`, but `ecs` must not depend on rendering crates.
- Feature crates may depend on `error` to emit `ErrorEvent` messages.
- Feature crates must use `error::GameError` and `error::Result` instead of defining local result aliases.
- Do not define another `Result` type alias in any crate.
- Avoid adding dependencies until the feature needs them.
- Keep the template compiling after every structural change.

## Common Tasks

- New shared error type or reporting behavior: add it under `crates/error/`.
- New ECS category: add a module under `crates/ecs/src/` and register its plugin in `crates/ecs/src/lib.rs`.
- New gameplay rule: add it under `crates/gameplay/src/rules/`.
- New gameplay flow/state behavior: add it under `crates/gameplay/src/flow/`.
- New spawning behavior: add it under `crates/gameplay/src/spawning/`.
- New character: add it under `crates/ecs/src/characters/`.
- New item: add it under `crates/ecs/src/items/`.
- New background or environment entity: add it under `crates/ecs/src/background/` or `crates/ecs/src/world/`.
- New 2D screen: add it under `crates/render_2d/src/screens/`, and add gameplay state in `crates/gameplay` if needed.
- New 2D UI feature: add it under `crates/render_2d/src/ui/`, reuse `theme.rs`, and keep visual constants centralized.
- New 3D camera, scene, mesh, light, or 3D UI feature: add it under `crates/render_3d/src/`.
- New asset type: create a matching folder under `assets/` and document naming rules in `docs/assets.md`.

## Verification

Run these before handing work back:

```sh
cargo fmt --check
cargo check
```
