use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleScreenTransition2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct ExampleFadeTransition2d {
    pub alpha: f32,
    pub seconds_remaining: f32,
}

#[derive(Bundle)]
pub struct ExampleFadeTransition2dBundle {
    pub marker: ExampleScreenTransition2d,
    pub fade: ExampleFadeTransition2d,
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl ExampleFadeTransition2dBundle {
    pub fn new(color: Color, alpha: f32, seconds_remaining: f32) -> Self {
        Self {
            marker: ExampleScreenTransition2d,
            fade: ExampleFadeTransition2d {
                alpha,
                seconds_remaining,
            },
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(color.with_alpha(alpha)),
        }
    }
}
