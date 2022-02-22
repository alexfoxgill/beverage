use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

use crate::player_vision::{PlayerVisibility, PlayerVisionUpdate, VisibilityMemory};

pub struct ActorRenderPlugin;
impl Plugin for ActorRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            actor_visibility.after(PlayerVisionUpdate),
        );
    }
}

fn actor_visibility(
    mut vis_query: Query<&mut Visibility>,
    query: Query<(Entity, &PlayerVisibility, Option<&Children>), Changed<PlayerVisibility>>,
) {
    for (entity, player_vis, children) in query.iter() {
        if let &PlayerVisibility {
            is_visible,
            memory: VisibilityMemory::Transient,
        } = player_vis
        {
            if let Ok(mut vis) = vis_query.get_mut(entity) {
                if vis.is_visible != is_visible {
                    vis.is_visible = is_visible;

                    if let Some(children) = children {
                        for &child in children.iter() {
                            if let Ok(mut vis) = vis_query.get_mut(child) {
                                vis.is_visible = is_visible;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_player() -> ShapeBundle {
    GeometryBuilder::build_as(
        &Circle {
            radius: 30.0,
            center: Vec2::new(0.0, 0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::WHITE),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        Transform::default(),
    )
}

pub fn render_enemy() -> ShapeBundle {
    GeometryBuilder::build_as(
        &Circle {
            radius: 30.0,
            center: Vec2::new(0.0, 0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        Transform::default(),
    )
}

pub fn direction_indicator() -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Polygon {
            points: vec![
                Vec2::new(-15.0, 30.0),
                Vec2::new(0.0, 45.0),
                Vec2::new(15.0, 30.0),
            ],
            closed: true,
        },
        DrawMode::Fill(FillMode::color(Color::YELLOW)),
        Transform::default(),
    )
}
