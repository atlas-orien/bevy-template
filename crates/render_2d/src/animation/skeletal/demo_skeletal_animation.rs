//! Demo 2D 骨骼动画边界，使用 Bevy 原生 Transform 层级表达骨骼。

use bevy::prelude::*;
use bevy::sprite::Anchor;

const DEMO_SKELETON_CYCLE_SECONDS: f32 = 1.2;
const DEMO_TORSO_SIZE: Vec2 = Vec2::new(18.0, 42.0);
const DEMO_ARM_SIZE: Vec2 = Vec2::new(8.0, 30.0);
const DEMO_JOINT_SIZE: Vec2 = Vec2::splat(13.0);
const DEMO_BONE_COLOR: Color = Color::srgb(0.38, 0.84, 0.95);
const DEMO_BONE_Z: f32 = 6.0;
const DEMO_JOINT_Z: f32 = 7.0;

pub struct DemoSkeleton2d {
    pub bundle: DemoSkeleton2dBundle,
    pub rig: DemoSkeleton2dRig,
}

impl DemoSkeleton2d {
    pub fn new(translation: Vec3, bone_image: Handle<Image>, joint_image: Handle<Image>) -> Self {
        Self {
            bundle: DemoSkeleton2dBundle::new(translation),
            rig: DemoSkeleton2dRig::new(bone_image, joint_image),
        }
    }

    pub fn into_bundle(self) -> impl Bundle {
        (self.bundle, self.rig.into_children())
    }
}

pub struct DemoSkeleton2dRig {
    bone_image: Handle<Image>,
    joint_image: Handle<Image>,
}

impl DemoSkeleton2dRig {
    fn new(bone_image: Handle<Image>, joint_image: Handle<Image>) -> Self {
        Self {
            bone_image,
            joint_image,
        }
    }

    fn into_children(self) -> impl Bundle {
        let bone_image = self.bone_image;
        let joint_image = self.joint_image;

        children![
            DemoBone2dBundle::torso(bone_image.clone()),
            Self::shoulder(joint_image.clone(), DemoJoint2d::LeftShoulder),
            Self::shoulder(joint_image.clone(), DemoJoint2d::RightShoulder),
            Self::arm(
                bone_image.clone(),
                joint_image.clone(),
                DemoSkeletonSide::Left
            ),
            Self::arm(bone_image, joint_image, DemoSkeletonSide::Right),
        ]
    }

    fn shoulder(image: Handle<Image>, joint: DemoJoint2d) -> DemoJoint2dBundle {
        DemoJoint2dBundle::new(image, joint, joint.translation())
    }

    fn arm(
        bone_image: Handle<Image>,
        joint_image: Handle<Image>,
        side: DemoSkeletonSide,
    ) -> impl Bundle {
        (
            DemoBone2dBundle::upper_arm(bone_image.clone(), side.upper_arm_bone(), side),
            children![
                DemoJoint2dBundle::new(
                    joint_image,
                    side.elbow_joint(),
                    side.elbow_joint().translation(),
                ),
                DemoBone2dBundle::lower_arm(bone_image, side.lower_arm_bone(), side,),
            ],
        )
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(super) struct DemoSkeleton2dRoot;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum DemoBone2d {
    Torso,
    LeftUpperArm,
    LeftLowerArm,
    RightUpperArm,
    RightLowerArm,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
enum DemoJoint2d {
    LeftShoulder,
    LeftElbow,
    RightShoulder,
    RightElbow,
}

impl DemoJoint2d {
    fn translation(self) -> Vec3 {
        match self {
            Self::LeftShoulder => Vec3::new(-15.0, 34.0, 0.0),
            Self::RightShoulder => Vec3::new(15.0, 34.0, 0.0),
            Self::LeftElbow | Self::RightElbow => Vec3::new(0.0, -26.0, 0.0),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub(super) struct DemoSkeletalAnimation2d {
    elapsed_seconds: f32,
    cycle_seconds: f32,
    swing_radians: f32,
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
    pub fn tick(&mut self, delta_seconds: f32) -> f32 {
        self.elapsed_seconds = (self.elapsed_seconds + delta_seconds) % self.cycle_seconds;
        self.pose_phase()
    }

    pub fn pose_phase(&self) -> f32 {
        (self.elapsed_seconds / self.cycle_seconds) * std::f32::consts::TAU
    }
}

#[derive(Bundle)]
pub struct DemoSkeleton2dBundle {
    marker: DemoSkeleton2dRoot,
    animation: DemoSkeletalAnimation2d,
    transform: Transform,
    visibility: Visibility,
}

impl DemoSkeleton2dBundle {
    fn new(translation: Vec3) -> Self {
        Self {
            marker: DemoSkeleton2dRoot,
            animation: DemoSkeletalAnimation2d::default(),
            transform: Transform::from_translation(translation),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
struct DemoBone2dBundle {
    bone: DemoBone2d,
    sprite: Sprite,
    anchor: Anchor,
    transform: Transform,
}

impl DemoBone2dBundle {
    fn torso(image: Handle<Image>) -> Self {
        Self {
            bone: DemoBone2d::Torso,
            sprite: bone_sprite(image, DEMO_TORSO_SIZE),
            anchor: Anchor::TOP_CENTER,
            transform: Transform::from_translation(Vec3::new(0.0, 18.0, DEMO_BONE_Z)),
        }
    }

    fn upper_arm(image: Handle<Image>, bone: DemoBone2d, side: DemoSkeletonSide) -> Self {
        Self {
            bone,
            sprite: bone_sprite(image, DEMO_ARM_SIZE),
            anchor: Anchor::TOP_CENTER,
            transform: Transform {
                translation: Vec3::new(side.x_sign() * 15.0, 34.0, DEMO_BONE_Z),
                rotation: Quat::from_rotation_z(side.rest_angle()),
                ..default()
            },
        }
    }

    fn lower_arm(image: Handle<Image>, bone: DemoBone2d, side: DemoSkeletonSide) -> Self {
        Self {
            bone,
            sprite: bone_sprite(image, DEMO_ARM_SIZE),
            anchor: Anchor::TOP_CENTER,
            transform: Transform {
                translation: Vec3::new(0.0, -26.0, 0.0),
                rotation: Quat::from_rotation_z(side.rest_angle() * 0.35),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
struct DemoJoint2dBundle {
    joint: DemoJoint2d,
    sprite: Sprite,
    transform: Transform,
}

impl DemoJoint2dBundle {
    fn new(image: Handle<Image>, joint: DemoJoint2d, translation: Vec3) -> Self {
        Self {
            joint,
            sprite: Sprite {
                image,
                custom_size: Some(DEMO_JOINT_SIZE),
                ..default()
            },
            transform: Transform::from_translation(translation.with_z(DEMO_JOINT_Z)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum DemoSkeletonSide {
    Left,
    Right,
}

impl DemoSkeletonSide {
    fn x_sign(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }

    fn rest_angle(self) -> f32 {
        self.x_sign() * 0.28
    }

    fn mirrored_swing(self, swing: f32) -> f32 {
        match self {
            Self::Left => swing,
            Self::Right => -swing,
        }
    }

    fn upper_arm_bone(self) -> DemoBone2d {
        match self {
            Self::Left => DemoBone2d::LeftUpperArm,
            Self::Right => DemoBone2d::RightUpperArm,
        }
    }

    fn lower_arm_bone(self) -> DemoBone2d {
        match self {
            Self::Left => DemoBone2d::LeftLowerArm,
            Self::Right => DemoBone2d::RightLowerArm,
        }
    }

    fn elbow_joint(self) -> DemoJoint2d {
        match self {
            Self::Left => DemoJoint2d::LeftElbow,
            Self::Right => DemoJoint2d::RightElbow,
        }
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

fn demo_bone_rotation(bone: DemoBone2d, swing: f32) -> Quat {
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

fn bone_sprite(image: Handle<Image>, size: Vec2) -> Sprite {
    Sprite {
        image,
        color: DEMO_BONE_COLOR,
        custom_size: Some(size),
        ..default()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

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
            .spawn(
                DemoSkeleton2d::new(Vec3::ZERO, Handle::default(), Handle::default()).into_bundle(),
            )
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
}
