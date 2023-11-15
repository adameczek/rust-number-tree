use crate::components::NumberNode;
use bevy::{prelude::*, window::PrimaryWindow};
pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_node);
    }
}

pub fn spawn_first_node(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        NumberNode { value: 0 },
        SpatialBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() - 40.0, 0.0),
            visibility: Visibility::Visible,
            ..default()
        },
    ));
}
