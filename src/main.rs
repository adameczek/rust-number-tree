pub mod components;
pub mod graphics;
pub mod numbernode;
mod cursor;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::NumberNode;
use graphics::GraphicsPlugin;
use numbernode::NodePlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                title: "Number Tree".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .register_type::<NumberNode>()
        .add_systems(Startup, setup)
        .add_plugins(NodePlugin)
        .add_plugins(GraphicsPlugin)
        .add_plugins(WorldInspectorPlugin::new()) // adds default options and `InspectorEguiImpl`s
        .run();
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
