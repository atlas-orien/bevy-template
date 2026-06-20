use bevy::prelude::*;

use super::{Pose3d, plugin::Skeleton3dPlugin};
use crate::primitives::skeletons::{Bone3d, BoneId3d, SkeletonId3d};

#[test]
fn pose_updates_matching_bone_transform() {
    let skeleton = SkeletonId3d(1);
    let bone = BoneId3d(7);
    let target = Transform::from_xyz(1.0, 2.0, 3.0);

    let mut app = App::new();
    app.add_plugins(Skeleton3dPlugin);

    let pose_entity = app
        .world_mut()
        .spawn(Pose3d::empty(skeleton).with_bone(bone, target))
        .id();
    let bone_entity = app
        .world_mut()
        .spawn((Bone3d::new(skeleton, bone), Transform::default()))
        .id();

    app.world_mut()
        .entity_mut(pose_entity)
        .insert(Pose3d::empty(skeleton).with_bone(bone, Transform::from_xyz(4.0, 5.0, 6.0)));

    app.update();

    let transform = app.world().entity(bone_entity).get::<Transform>().unwrap();
    assert_eq!(transform.translation, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn pose_ignores_bones_from_other_skeletons() {
    let pose_skeleton = SkeletonId3d(1);
    let other_skeleton = SkeletonId3d(2);
    let bone = BoneId3d(7);

    let mut app = App::new();
    app.add_plugins(Skeleton3dPlugin);

    app.world_mut()
        .spawn(Pose3d::empty(pose_skeleton).with_bone(bone, Transform::from_xyz(4.0, 5.0, 6.0)));
    let bone_entity = app
        .world_mut()
        .spawn((Bone3d::new(other_skeleton, bone), Transform::default()))
        .id();

    app.update();

    let transform = app.world().entity(bone_entity).get::<Transform>().unwrap();
    assert_eq!(transform.translation, Vec3::ZERO);
}
