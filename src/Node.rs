use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_node);
    }
}

#[derive(Component)]
pub struct Node(i32);

pub fn spawn_node(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let font = asset_server.load("Roboto-Regular.ttf");

    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() - 40.0, 0.0),
            visibility: Visibility::Visible,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform {
                    scale: Vec3::splat(64.),
                    ..default()
                },
                ..default()
            });
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "20",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
        });
    // commands.spawn((MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     transform: Transform::from_xyz(window.width() / 2.0, window.height() - 40.0, 0.0)
    //         .with_scale(Vec3::splat(64.)),
    //     ..default()
    // },));
    // commands.spawn(Text2dBundle {
    //     text: Text::from_section(
    //         "20",
    //         TextStyle {
    //             font: font.clone(),
    //             font_size: 16.0,
    //             color: Color::WHITE,
    //         },
    //     )
    //     .with_alignment(TextAlignment::Center),
    //     transform: Transform::from_xyz(window.width() / 2.0, window.height() - 40.0, 0.0),
    //     ..default()
    // });
}
