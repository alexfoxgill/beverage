use std::{
    f32::consts::PI,
    ops::{Add, Neg, Sub},
};

use hex2d::Coordinate;

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Radians(f32);

impl Radians {
    pub fn abs(self) -> Self {
        Radians(self.0.abs())
    }
}

impl Neg for Radians {
    type Output = Radians;

    fn neg(self) -> Self::Output {
        Radians(-self.0)
    }
}

impl Sub for Radians {
    type Output = Radians;

    fn sub(self, rhs: Self) -> Self::Output {
        Radians(self.0 - rhs.0)
    }
}

impl Add for Radians {
    type Output = Radians;

    fn add(self, rhs: Self) -> Self::Output {
        Radians(self.0 + rhs.0)
    }
}

pub const RADIANS_0DEG: Radians = Radians(0.0);
pub const RADIANS_60DEG: Radians = Radians(60.0 * PI / 180.0);
pub const RADIANS_120DEG: Radians = Radians(120.0 * PI / 180.0);
pub const RADIANS_180DEG: Radians = Radians(PI);

const SQRT_3: f32 = 1.73205080757;

pub fn radians_from_yz(c: Coordinate) -> Radians {
    let q = c.x as f32;
    let r = c.z() as f32;

    let x = 3.0 / 2.0 * q;
    let y = SQRT_3 * (r + q / 2.0);

    Radians(x.atan2(-y))
}

#[cfg(test)]
mod tests {
    use hex2d::Angle;

    use crate::domain::common::HexDirection;

    use super::*;

    #[test]
    fn test_radians() {
        let inputs = [
            (Angle::Forward, RADIANS_0DEG),
            (Angle::Left, -RADIANS_60DEG),
            (Angle::Right, RADIANS_60DEG),
            (Angle::LeftBack, -RADIANS_120DEG),
            (Angle::RightBack, RADIANS_120DEG),
            (Angle::Back, RADIANS_180DEG),
        ];

        for (angle, rads) in inputs {
            let c = Coordinate::new(0, 0) + (HexDirection::YZ + angle);

            let res = radians_from_yz(c);

            assert!((res - rads).0 < f32::EPSILON, "{angle:?}")
        }
    }
}
