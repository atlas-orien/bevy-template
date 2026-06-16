use std::time::Duration;

use bevy::prelude::*;

use super::DemoSkeleton2d;
use super::rig::{DemoBone2d, DemoBone2dBundle, DemoSkeletonSide};
use super::systems::{
    DEMO_SKELETON_CYCLE_SECONDS, DemoSkeletalAnimation2d, demo_bone_rotation,
    demo_skeletal_animation_system,
};

fn skeletal_app(delta_seconds: f32) -> App {
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(delta_seconds));
    app.insert_resource(time)
        .add_systems(Update, demo_skeletal_animation_system);
    app
}

#[test]
fn animation_tick_wraps_inside_cycle() {
    let mut animation = DemoSkeletalAnimation2d {
        elapsed_seconds: 1.1,
        cycle_seconds: 1.2,
        swing_radians: 0.2,
    };

    animation.tick(0.3);

    assert!((animation.elapsed_seconds - 0.2).abs() < 0.0001);
}

#[test]
fn mirrored_arm_rotations_move_in_opposite_directions() {
    let swing = 0.25;
    let left = demo_bone_rotation(DemoBone2d::LeftUpperArm, swing);
    let right = demo_bone_rotation(DemoBone2d::RightUpperArm, swing);

    assert_ne!(left, right);
}

#[test]
fn skeletal_system_updates_child_bone_rotation() {
    let mut app = skeletal_app(DEMO_SKELETON_CYCLE_SECONDS / 4.0);
    let skeleton = app
        .world_mut()
        .spawn(DemoSkeleton2d::new(Vec3::ZERO, Handle::default(), Handle::default()).into_bundle())
        .id();
    let bone = app
        .world_mut()
        .spawn(DemoBone2dBundle::upper_arm(
            Handle::default(),
            DemoBone2d::LeftUpperArm,
            DemoSkeletonSide::Left,
        ))
        .id();
    app.world_mut().entity_mut(skeleton).add_child(bone);

    app.update();

    let transform = app.world().get::<Transform>(bone).unwrap();
    assert_eq!(
        transform.rotation,
        demo_bone_rotation(
            DemoBone2d::LeftUpperArm,
            DemoSkeletalAnimation2d::default().swing_radians
        )
    );
}
