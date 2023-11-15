use bevy::{
    prelude::{shape::Circle, *},
    sprite::MaterialMesh2dBundle,
};

use crate::components::NumberNode;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_number_node_renderer, update_number_node_text_value),
        );
    }
}

fn update_number_node_text_value(
    mut texts: Query<(&mut Text, &Parent)>,
    number_nodes: Query<&NumberNode>,
) {
    for (mut text, parent) in texts.iter_mut() {
        let parent_number_node = number_nodes.get(parent.get()).unwrap();
        text.sections[0].value = parent_number_node.value.to_string();
    }
}

fn spawn_number_node_renderer(
    mut commands: Commands,
    query: Query<(Entity, &NumberNode), Added<NumberNode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("Roboto-Regular.ttf");

    for (entity, number_node) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(Circle {
                        radius: 32.0,
                        vertices: 8,
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform { ..default() },
                ..default()
            });
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    number_node.value.to_string(),
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
    }
}
