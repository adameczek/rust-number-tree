use bevy::prelude::*;

pub struct CursorPlugin;

#[derive(Component)]
pub struct Cursor;

#[derive(Default)]
struct CursorState {
    cursor_world: Vec2,
    cursor_moved: bool,
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor_entity)
            .add_systems(Update, update_number_node_text_value);
    }
}

fn cursor_state(
    mut state: ResMut<State>,
    e_cursor_moved: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    mut q_cursor_state: Query<&mut CursorState>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    let event_cursor_screen = state.er_cursor_moved.latest(&e_cursor_moved);

    for mut cursor_state in q_cursor_state.iter_mut() {
        if let Some(event_cursor_screen) = event_cursor_screen {
            let window = windows.get_primary().unwrap();
            let cam_transform = q_camera.iter().last().unwrap();
            cursor_state.cursor_world =
                cursor_to_world(window, cam_transform, event_cursor_screen.position);

            cursor_state.cursor_moved = true;
        } else {
            cursor_state.cursor_moved = false;
        }
    }
}

fn cursor_transform(
    commands: &mut Commands,
    q_cursor_state: Query<&CursorState>,
    mut q_cursor: Query<(Entity, &mut Transform), With<Cursor>>,
) {
    let cursor_state = q_cursor_state.iter().next().unwrap();

    for (cursor_e, mut transform) in q_cursor.iter_mut() {
        transform.translation.x = cursor_state.cursor_world.x;
        transform.translation.y = cursor_state.cursor_world.y;
        commands.remove_one::<Parent>(cursor_e);
    }
}

fn cursor_to_world(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
    // get the size of the window
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let screen_pos = cursor_pos - size / 2.0;

    // apply the camera transform
    let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);
    Vec2::new(out.x, out.y)
}
