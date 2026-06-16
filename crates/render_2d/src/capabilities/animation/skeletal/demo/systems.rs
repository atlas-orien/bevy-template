use bevy::prelude::*;

use super::rig::{DemoBone2d, DemoSkeleton2dRoot, DemoSkeletonSide};

pub(super) const DEMO_SKELETON_CYCLE_SECONDS: f32 = 1.2;

pub(in crate::capabilities::animation::skeletal) struct DemoSkeletalAnimationPlugin;

impl Plugin for DemoSkeletalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_skeletal_animation_system);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub(super) struct DemoSkeletalAnimation2d {
    pub(super) elapsed_seconds: f32,
    pub(super) cycle_seconds: f32,
    pub(super) swing_radians: f32,
}

impl Default for DemoSkeletalAnimation2d {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            cycle_seconds: DEMO_SKELETON_CYCLE_SECONDS,
            swing_radians: std::f32::consts::FRAC_PI_8,
        }
    }
}

impl DemoSkeletalAnimation2d {
    pub(super) fn tick(&mut self, delta_seconds: f32) -> f32 {
        self.elapsed_seconds = (self.elapsed_seconds + delta_seconds) % self.cycle_seconds;
        self.pose_phase()
    }

    fn pose_phase(&self) -> f32 {
        (self.elapsed_seconds / self.cycle_seconds) * std::f32::consts::TAU
    }
}

pub(super) fn demo_skeletal_animation_system(
    time: Res<Time>,
    mut skeletons: Query<(Entity, &mut DemoSkeletalAnimation2d), With<DemoSkeleton2dRoot>>,
    parents: Query<&ChildOf>,
    mut bones: Query<(Entity, &DemoBone2d, &mut Transform)>,
) {
    for (skeleton_entity, mut animation) in &mut skeletons {
        let phase = animation.tick(time.delta_secs());
        let swing = phase.sin() * animation.swing_radians;

        for (bone_entity, bone, mut transform) in &mut bones {
            if !has_skeleton_ancestor(bone_entity, skeleton_entity, &parents) {
                continue;
            }

            transform.rotation = demo_bone_rotation(*bone, swing);
        }
    }
}

pub(super) fn demo_bone_rotation(bone: DemoBone2d, swing: f32) -> Quat {
    let angle = match bone {
        DemoBone2d::Torso => swing * 0.18,
        DemoBone2d::LeftUpperArm => DemoSkeletonSide::Left.rest_angle() + swing,
        DemoBone2d::LeftLowerArm => DemoSkeletonSide::Left.mirrored_swing(swing * 0.75),
        DemoBone2d::RightUpperArm => DemoSkeletonSide::Right.rest_angle() - swing,
        DemoBone2d::RightLowerArm => DemoSkeletonSide::Right.mirrored_swing(swing * 0.75),
    };

    Quat::from_rotation_z(angle)
}

fn has_skeleton_ancestor(
    entity: Entity,
    skeleton_entity: Entity,
    parents: &Query<&ChildOf>,
) -> bool {
    let mut current = entity;
    while let Ok(parent) = parents.get(current) {
        if parent.parent() == skeleton_entity {
            return true;
        }
        current = parent.parent();
    }

    false
}
