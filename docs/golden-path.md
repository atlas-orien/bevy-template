# Golden Path: Entity Movement Request

This is the canonical rule chain for adding movement-like behavior. Follow the
same ownership boundaries when adding new features.

## Flow

```text
external source
-> external_runtime::manager::set_movement_intent(id, target)
-> gameplay::api::RuntimeRequest::SetMovementIntent
-> gameplay::api::systems::forward_manager_requests_system
-> gameplay::api::systems::consume_gameplay_requests_system
-> prefab::identity::find_gameplay_entity(id)
-> intent::movement::set_movement_intent(entity, target, query)
-> optional navigation target/path/follower systems
-> ecs::systems::movement::movement_system
-> Transform changes
-> render_2d displays the entity
```

## Rules

- The template does not spawn a default object.
- Concrete projects decide which prefab owns which `GameplayEntityId`.
- Local input is polled in `external_runtime::input::local`.
- AI, scripts, replay, network, and device input are external sources.
- External sources submit requests through `external_runtime::manager`; they do
  not touch Bevy `Entity`, `Commands`, ECS components, physics, or render types.
- `gameplay::api` receives requests and is the only place that maps a public id
  back to a Bevy entity for execution.
- `intent` only writes intent data.
- `navigation` owns path/query/follower data when an object needs path-based
  movement.
- `ecs::systems` performs the world rule that moves `Transform`.
- `render_2d` reads world state and presents it.

## Adding A Similar Feature

1. Put external source polling or decision logic in `external_runtime::input`.
2. Add or reuse a manager API function that submits a `RuntimeRequest`.
3. Define request message data in `gameplay::api::runtime_channel::message`.
4. Consume and execute the request in `gameplay::api::systems`.
5. If the request targets an entity, use gameplay-facing ids, not raw `Entity`.
6. Put concrete object composition in `prefab`.
7. Put ECS data in `ecs::components`, global state in `ecs::resources`, and world
   rules in `ecs::systems`.
8. Keep render, physics, and navigation as presentation/foundation layers, not control
   sources.

## Do Not Do This

- Do not read keyboard, mouse, gamepad, AI, script, replay, or network sources in
  `gameplay`, `intent`, `ecs`, `prefab`, `render_2d`, or `render_3d`.
- Do not spawn gameplay entities by assembling loose component tuples outside
  `prefab`.
- Do not expose raw Bevy `Entity` through external runtime manager APIs.
- Do not import Rapier directly outside `crates/physics`.
