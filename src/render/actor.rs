use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

use crate::domain::common::{Facing, HexPos, HEX_SPACING};

use super::player_vision::{PlayerVisibility, PlayerVisionUpdate, VisibilityMemory};

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

fn transform(pos: &HexPos, facing: &Facing) -> Transform {
    Transform {
        translation: pos.as_translation(HEX_SPACING),
        rotation: facing.as_rotation(),
        ..Transform::default()
    }
}

pub fn render_player(pos: &HexPos, facing: &Facing) -> ShapeBundle {
    GeometryBuilder::new()
        .add(&Circle {
            radius: 30.0,
            center: Vec2::new(0.0, 0.0),
        })
        .add(&shapes::Polygon {
            points: vec![
                Vec2::new(-15.0, 30.0),
                Vec2::new(0.0, 45.0),
                Vec2::new(15.0, 30.0),
            ],
            closed: true,
        })
        .build(
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            transform(pos, facing),
        )
}

pub fn render_enemy(pos: &HexPos, facing: &Facing) -> ShapeBundle {
    GeometryBuilder::new()
        .add(&Circle {
            radius: 30.0,
            center: Vec2::new(0.0, 0.0),
        })
        .add(&shapes::Polygon {
            points: vec![
                Vec2::new(-15.0, 30.0),
                Vec2::new(0.0, 45.0),
                Vec2::new(15.0, 30.0),
            ],
            closed: true,
        })
        .build(
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            transform(pos, facing),
        )
}
