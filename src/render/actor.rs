use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

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
