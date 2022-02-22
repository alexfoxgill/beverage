use bevy::prelude::*;
use hex2d::{Coordinate, Position};

use crate::maths::{radians_from_yz, Radians};

use super::common::HexDirection;

pub enum VisionType {
    Radial(i32),
    Intersection(Box<VisionType>, Box<VisionType>),
    Union(Box<VisionType>, Box<VisionType>),
    Negative(Box<VisionType>),
    Conical(Radians),
    Obstructable,
}

impl VisionType {
    pub fn can_see<F: Fn(Coordinate) -> bool>(&self, point: Coordinate, is_obstacle: &F) -> bool {
        match self {
            VisionType::Radial(radius) => {
                point.x.abs().max(point.y.abs()).max(point.z().abs()) <= *radius
            }
            VisionType::Intersection(a, b) => {
                a.can_see(point, is_obstacle) && b.can_see(point, is_obstacle)
            }
            VisionType::Union(a, b) => {
                a.can_see(point, is_obstacle) || b.can_see(point, is_obstacle)
            }
            VisionType::Negative(a) => !a.can_see(point, is_obstacle),
            VisionType::Conical(view_width) => {
                if point == Coordinate::new(0, 0) {
                    return true;
                }

                let angle_to_point = radians_from_yz(point).abs();

                angle_to_point <= *view_width
            }
            VisionType::Obstructable => {
                let line = Coordinate::new(0, 0).line_to_iter(point);

                for x in line {
                    if point == x {
                        return true;
                    }
                    if is_obstacle(x) {
                        return false;
                    }
                }

                true
            }
        }
    }

    pub fn and(self, other: VisionType) -> VisionType {
        VisionType::Intersection(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: VisionType) -> VisionType {
        VisionType::Union(Box::new(self), Box::new(other))
    }
}

#[derive(Component)]
pub struct Vision {
    pub vision: VisionType,
}

impl Vision {
    pub fn new(vision: VisionType) -> Vision {
        Vision { vision }
    }

    pub fn new_radial(radius: i32) -> Vision {
        Vision {
            vision: VisionType::Radial(radius),
        }
    }

    pub fn can_see_relative<F: Fn(Coordinate) -> bool>(
        &self,
        pos: Position,
        point: Coordinate,
        is_obstacle: F,
    ) -> bool {
        let relative_point = translate_to_relative(pos, point);
        self.vision.can_see(relative_point, &|x| {
            is_obstacle(translate_from_relative(pos, x))
        })
    }
}

fn translate_to_relative(pos: Position, point: Coordinate) -> Coordinate {
    (point - pos.coord).rotate_around_zero(HexDirection::YZ - pos.dir)
}

fn translate_from_relative(pos: Position, point: Coordinate) -> Coordinate {
    let diff = pos.dir - HexDirection::YZ;
    point.rotate_around_zero(diff) + pos.coord
}
