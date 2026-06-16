//! Render capabilities 的开发预览入口。

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use render_2d::capabilities::effects::DemoFlashEffect2d;
use render_2d::capabilities::effects::EffectsPlugin;
use render_2d::capabilities::lighting::DemoGlow2d;
use render_2d::capabilities::materials::DemoColorMaterial2d;
use render_2d::capabilities::mesh::DemoMesh2d;
use render_2d::capabilities::pixel::DemoPixelSnap2d;
use render_2d::capabilities::pixel::PixelPlugin;
use render_2d::primitives::camera::FixedCamera2dBundle;

const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 540.0;
const FLASH_SPAWN_SECONDS: f32 = 0.42;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dev Preview - Render Capabilities".to_string(),
                        resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((EffectsPlugin, PixelPlugin))
        .add_systems(Startup, spawn_capabilities_preview_system)
        .add_systems(Update, spawn_repeating_flash_effect_system)
        .run();
}

fn spawn_capabilities_preview_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(FixedCamera2dBundle::default());
    spawn_stage(&mut commands);
    spawn_lighting_demo(&mut commands);
    spawn_mesh_material_demo(&mut commands, &mut meshes, &mut materials);
    spawn_pixel_demo(&mut commands);
    spawn_effect_demo(&mut commands);
}

fn spawn_stage(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.08, 0.1, 0.12),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.17, 0.2),
            custom_size: Some(Vec2::new(760.0, 300.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -12.0, -9.0),
    ));

    for x in [-320.0, -160.0, 0.0, 160.0, 320.0] {
        commands.spawn((
            Sprite {
                color: Color::srgba(0.8, 0.86, 0.92, 0.16),
                custom_size: Some(Vec2::new(2.0, 300.0)),
                ..default()
            },
            Transform::from_xyz(x, -12.0, -8.0),
        ));
    }
}

fn spawn_lighting_demo(commands: &mut Commands) {
    commands.spawn(DemoGlow2d::new(Vec3::new(-260.0, 60.0, 0.0)));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.64, 0.94),
            custom_size: Some(Vec2::splat(34.0)),
            ..default()
        },
        Transform::from_xyz(-260.0, 60.0, 3.0),
    ));
}

fn spawn_mesh_material_demo(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let mesh = meshes.add(Triangle2d::new(
        Vec2::new(0.0, 72.0),
        Vec2::new(-72.0, -56.0),
        Vec2::new(72.0, -56.0),
    ));
    let material = materials.add(Color::srgb(0.98, 0.48, 0.26));

    commands.spawn((
        DemoMesh2d::new(mesh, Vec3::new(0.0, 38.0, 1.0)),
        DemoColorMaterial2d::new(material),
    ));
}

fn spawn_pixel_demo(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.34, 0.96, 0.64),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(235.35, 67.65, 2.0),
        DemoPixelSnap2d::new(1.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgba(0.34, 0.96, 0.64, 0.25),
            custom_size: Some(Vec2::new(58.0, 58.0)),
            ..default()
        },
        Transform::from_xyz(235.35, 67.65, 1.0),
    ));
}

fn spawn_effect_demo(commands: &mut Commands) {
    for x in [-64.0, 0.0, 64.0] {
        commands.spawn(DemoFlashEffect2d::new(Vec3::new(x, -118.0, 0.0)));
    }
}

fn spawn_repeating_flash_effect_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    let timer =
        timer.get_or_insert_with(|| Timer::from_seconds(FLASH_SPAWN_SECONDS, TimerMode::Repeating));

    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    commands.spawn(DemoFlashEffect2d::new(Vec3::new(0.0, -118.0, 0.0)));
}
