use bevy::prelude::*;

use crate::capabilities::skeleton::Pose3d;
use crate::primitives::skeletons::Bone3d;

pub(super) fn apply_pose_to_bones_system(
    poses: Query<&Pose3d, Changed<Pose3d>>,
    mut bones: Query<(&Bone3d, &mut Transform)>,
) {
    for pose in &poses {
        for bone_pose in &pose.bones {
            if bone_pose.skeleton != pose.skeleton {
                continue;
            }

            for (bone, mut transform) in &mut bones {
                if bone.skeleton == pose.skeleton && bone.id == bone_pose.bone {
                    *transform = bone_pose.local_transform;
                }
            }
        }
    }
}
