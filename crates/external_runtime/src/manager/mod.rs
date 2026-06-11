mod state;
mod transport;
pub mod user;

pub use self::user::{
    ExternalRuntimeManager, change_state, clear_session, despawn_object_entities,
    despawn_user_entities, entities, entities_for_object, entities_for_user, has_object_entity,
    has_user_entity, set_object_movement_intent, set_user_movement_intent, spawn_prefab,
    spawn_prefab_for_object, spawn_prefab_for_user, spawn_prefab_for_user_object,
    spawn_prefab_with_context,
};
pub use gameplay::api::{
    RuntimeEntityRegistrationMessage, RuntimeObjectId, RuntimeSpawnContext, RuntimeUserId,
};
