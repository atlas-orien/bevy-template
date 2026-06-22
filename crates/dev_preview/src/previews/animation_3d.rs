//! 3D animation preview with a glTF character scene.

use bevy::asset::AssetPlugin;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use catalog::world_3d::{DemoFox3d, DemoPreviewCamera3d, DemoPreviewFloor3d, DemoPreviewLights3d};
use prefab::Prefab;
use render_2d::primitives::camera::UiCamera;
use render_3d::Render3dPlugin;
use render_3d::capabilities::animation::demo::{
    DemoFox3dAnimationState, DemoFox3dAnimationStateSet,
};
use render_3d::primitives::camera::OrbitCamera3d;

const PANEL_LEFT_PX: f32 = 24.0;
const PANEL_TOP_PX: f32 = 24.0;
const PANEL_GAP_PX: f32 = 8.0;
const BUTTON_WIDTH_PX: f32 = 118.0;
const BUTTON_HEIGHT_PX: f32 = 38.0;
const BUTTON_FONT_SIZE: f32 = 16.0;
const CAMERA_YAW_SPEED: f32 = 1.8;
const CAMERA_PITCH_SPEED: f32 = 1.2;
const CAMERA_MOUSE_SPEED: f32 = 0.006;
const CAMERA_ZOOM_SPEED: f32 = 0.45;
const CAMERA_MIN_PITCH: f32 = -0.15;
const CAMERA_MAX_PITCH: f32 = 1.1;
const CAMERA_MIN_RADIUS: f32 = 3.0;
const CAMERA_MAX_RADIUS: f32 = 12.0;

const ANIMATION_BUTTONS: [AnimationButtonSpec; 3] = [
    AnimationButtonSpec {
        name: "animation-preview-idle",
        label: "Idle",
        state: DemoFox3dAnimationState::Idle,
    },
    AnimationButtonSpec {
        name: "animation-preview-walk",
        label: "Walk",
        state: DemoFox3dAnimationState::Walk,
    },
    AnimationButtonSpec {
        name: "animation-preview-run",
        label: "Run",
        state: DemoFox3dAnimationState::Run,
    },
];

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "../../assets".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dev Preview - 3D Animation".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(Render3dPlugin)
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 0.0,
            affects_lightmapped_meshes: true,
        })
        .add_systems(Startup, spawn_3d_animation_preview_system)
        .add_systems(
            Update,
            (
                switch_demo_fox_animation_with_keyboard_system,
                switch_demo_fox_animation_with_buttons_system,
                control_animation_preview_camera_system,
            ),
        )
        .run();
}

fn spawn_3d_animation_preview_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let animations = DemoFox3d::animations(&asset_server, &mut animation_graphs);
    DemoPreviewCamera3d::orbit_prefab().spawn(&mut commands);
    commands.spawn(UiCamera::default());
    DemoPreviewLights3d::prefab().spawn(&mut commands);
    DemoPreviewFloor3d::prefab(&mut meshes, &mut materials).spawn(&mut commands);
    DemoFox3d::prefab_with_scene(
        &asset_server,
        DemoFox3dAnimationState::Idle,
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        animations,
    )
    .spawn(&mut commands);

    spawn_animation_preview_controls(&mut commands);
}

fn switch_demo_fox_animation_with_keyboard_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut foxes: Query<&mut DemoFox3dAnimationStateSet>,
) {
    let requested_state = if keyboard_input.just_pressed(KeyCode::Digit1) {
        Some(DemoFox3dAnimationState::Idle)
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        Some(DemoFox3dAnimationState::Walk)
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        Some(DemoFox3dAnimationState::Run)
    } else {
        None
    };

    let Some(next_state) = requested_state else {
        return;
    };

    set_demo_fox_animation_state(next_state, &mut foxes);
}

fn switch_demo_fox_animation_with_buttons_system(
    mut interactions: Query<(&Interaction, &Name, &mut BackgroundColor), Changed<Interaction>>,
    mut foxes: Query<&mut DemoFox3dAnimationStateSet>,
) {
    for (interaction, name, mut background) in &mut interactions {
        let Some(next_state) = animation_state_from_button_name(name.as_str()) else {
            continue;
        };

        match *interaction {
            Interaction::Pressed => {
                background.0 = Color::srgb(0.25, 0.44, 0.72);
                set_demo_fox_animation_state(next_state, &mut foxes);
            }
            Interaction::Hovered => {
                background.0 = Color::srgb(0.18, 0.28, 0.44);
            }
            Interaction::None => {
                background.0 = Color::srgb(0.10, 0.13, 0.18);
            }
        }
    }
}

fn control_animation_preview_camera_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut cameras: Query<&mut OrbitCamera3d>,
) {
    let mut yaw_delta = 0.0;
    let mut pitch_delta = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        yaw_delta += CAMERA_YAW_SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        yaw_delta -= CAMERA_YAW_SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        pitch_delta += CAMERA_PITCH_SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        pitch_delta -= CAMERA_PITCH_SPEED * time.delta_secs();
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        for motion in mouse_motion.read() {
            yaw_delta -= motion.delta.x * CAMERA_MOUSE_SPEED;
            pitch_delta += motion.delta.y * CAMERA_MOUSE_SPEED;
        }
    } else {
        mouse_motion.clear();
    }

    let zoom_delta = mouse_wheel
        .read()
        .fold(0.0, |total, wheel| total - wheel.y * CAMERA_ZOOM_SPEED);

    if yaw_delta == 0.0 && pitch_delta == 0.0 && zoom_delta == 0.0 {
        return;
    }

    for mut camera in &mut cameras {
        camera.yaw += yaw_delta;
        camera.pitch = (camera.pitch + pitch_delta).clamp(CAMERA_MIN_PITCH, CAMERA_MAX_PITCH);
        camera.radius = (camera.radius + zoom_delta).clamp(CAMERA_MIN_RADIUS, CAMERA_MAX_RADIUS);
    }
}

fn set_demo_fox_animation_state(
    next_state: DemoFox3dAnimationState,
    foxes: &mut Query<&mut DemoFox3dAnimationStateSet>,
) {
    for mut state in foxes {
        state.set(next_state);
    }
}

fn spawn_animation_preview_controls(commands: &mut Commands) {
    commands
        .spawn((
            Name::new("Animation Preview Controls"),
            Node {
                position_type: PositionType::Absolute,
                left: px(PANEL_LEFT_PX),
                top: px(PANEL_TOP_PX),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: px(PANEL_GAP_PX),
                ..default()
            },
        ))
        .with_children(|panel| {
            for button in ANIMATION_BUTTONS {
                panel
                    .spawn((
                        Button,
                        Name::new(button.name),
                        Node {
                            width: px(BUTTON_WIDTH_PX),
                            height: px(BUTTON_HEIGHT_PX),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.10, 0.13, 0.18)),
                    ))
                    .with_children(|button_entity| {
                        button_entity.spawn((
                            Text::new(button.label),
                            TextFont::from_font_size(BUTTON_FONT_SIZE),
                            TextColor(Color::srgb(0.95, 0.96, 0.98)),
                        ));
                    });
            }
        });
}

#[derive(Debug, Clone, Copy)]
struct AnimationButtonSpec {
    name: &'static str,
    label: &'static str,
    state: DemoFox3dAnimationState,
}

fn animation_state_from_button_name(name: &str) -> Option<DemoFox3dAnimationState> {
    for button in ANIMATION_BUTTONS {
        if button.name == name {
            return Some(button.state);
        }
    }

    None
}
