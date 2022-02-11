use std::{collections::VecDeque, iter, time::Duration};

use bevy::prelude::*;

use bevy_easings::{Ease, EaseFunction, EasingType, EasingsPlugin};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};
use hex2d::{Direction as HexDirection, *};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

pub const HEX_SPACING: Spacing = Spacing::FlatTop(40.0);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ControlSource {
    Player,
    AI,
}

#[derive(Component)]
pub struct Actor {
    pub control_source: ControlSource,
    pub actions_per_turn: u8,
    pub actions_remaining: u8,
}

#[derive(Component)]
pub struct Facing(pub HexDirection);

impl Facing {
    pub fn rotated(&self, angle: Angle) -> HexDirection {
        self.0 + angle
    }

    pub fn as_rotation(&self) -> Quat {
        // the z-axis points towards the camera, so rotate a negative amount
        Quat::from_rotation_z(-self.0.to_radians_flat::<f32>())
    }
}

#[derive(Component)]
pub struct HexPos(pub Coordinate);

impl HexPos {
    pub fn get_facing(&self, dir: HexDirection) -> Coordinate {
        self.0 + dir
    }

    pub fn move_facing(&mut self, dir: HexDirection) {
        self.0 = self.get_facing(dir);
    }

    pub fn as_translation(&self, spacing: Spacing) -> Vec3 {
        let (x, y) = self.0.to_pixel(spacing);
        // the y-axis points upward, so invert it
        Vec3::new(x, -y, 0.0)
    }
}

impl Default for HexPos {
    fn default() -> Self {
        Self(Coordinate::new(0, 0))
    }
}
