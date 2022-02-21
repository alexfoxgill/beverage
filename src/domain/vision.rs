use std::cmp::Ordering;

use bevy::prelude::*;
use hex2d::Coordinate;

use super::common::HexDirection;

#[derive(Component)]
pub struct Vision {
    pub radius: i32,
}

impl Vision {
    pub fn new(radius: i32) -> Vision {
        Vision { radius }
    }
}

pub struct ViewCone {
    from: Coordinate,
    left_dir: HexDirection,
    right_dir: HexDirection,
    radius: i32,
}

impl ViewCone {
    pub fn can_see<F: Fn(Coordinate) -> bool>(&self, point: Coordinate, is_obstacle: F) -> bool {
        if self.from == point {
            return true;
        }

        if self.from.distance(point) > self.radius {
            return false;
        }

        let offset = point - self.from;
        let left_side = RelativeSide::side_relative_to(self.left_dir, offset);
        if left_side == RelativeSide::Left {
            return false;
        }

        let right_side = RelativeSide::side_relative_to(self.right_dir, offset);
        if right_side == RelativeSide::Right {
            return false;
        }

        let line = self.from.line_to_iter(point);

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RelativeSide {
    Right,
    Left,
    On,
}

impl RelativeSide {
    fn from(i: i32) -> RelativeSide {
        match i.cmp(&0) {
            Ordering::Less => RelativeSide::Left,
            Ordering::Equal => RelativeSide::On,
            Ordering::Greater => RelativeSide::Right,
        }
    }

    pub fn side_relative_to(dir: HexDirection, point: Coordinate) -> RelativeSide {
        RelativeSide::from(match dir {
            HexDirection::YZ => point.x,
            HexDirection::ZY => -point.x,

            HexDirection::XZ => -point.y,
            HexDirection::ZX => point.y,

            HexDirection::XY => point.z(),
            HexDirection::YX => -point.z(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex2d::Angle;
    use rand::*;

    fn random_coordinate() -> Coordinate {
        let mut rng = thread_rng();
        Coordinate::<i32>::new(rng.gen_range(-10..10), rng.gen_range(-10..10))
    }

    fn all_directions(f: fn(HexDirection) -> ()) {
        for d in HexDirection::all().iter() {
            f(*d)
        }
    }

    #[test]
    fn view_cone_can_see_directly_ahead() {
        all_directions(|dir| {
            let from = random_coordinate();
            let cone = ViewCone {
                from,
                left_dir: dir,
                right_dir: dir,
                radius: 1,
            };

            let point = from + dir;

            let res = cone.can_see(point, |_| false);

            assert!(res, "from = {from:?}, dir = {dir:?}")
        });
    }

    #[test]
    fn view_cone_cannot_see_to_left() {
        all_directions(|dir| {
            let from = random_coordinate();
            let cone = ViewCone {
                from,
                left_dir: dir,
                right_dir: dir,
                radius: 1,
            };

            let point = cone.from + (dir + Angle::Left);

            let res = cone.can_see(point, |_| false);

            assert!(!res, "from = {from:?}, dir = {dir:?}")
        })
    }

    #[test]
    fn view_cone_cannot_see_to_right() {
        all_directions(|dir| {
            let from = random_coordinate();
            let cone = ViewCone {
                from,
                left_dir: dir,
                right_dir: dir,
                radius: 1,
            };

            let point = from + (dir + Angle::Right);

            let res = cone.can_see(point, |_| false);

            assert!(!res, "from = {from:?}, dir = {dir:?}")
        })
    }

    #[test]
    fn view_cone_can_see_on_left_bound() {
        all_directions(|dir| {
            let from = random_coordinate();
            let cone = ViewCone {
                from,
                left_dir: dir + Angle::Left,
                right_dir: dir,
                radius: 1,
            };
            let point = cone.from + (dir + Angle::Left);

            let res = cone.can_see(point, |_| false);

            assert!(res, "from = {from:?}, dir = {dir:?}")
        })
    }

    #[test]
    fn view_cone_can_see_on_right_bound() {
        all_directions(|dir| {
            let from = random_coordinate();
            let cone = ViewCone {
                from,
                left_dir: dir,
                right_dir: dir + Angle::Right,
                radius: 1,
            };

            let point = cone.from + (dir + Angle::Right);

            let res = cone.can_see(point, |_| false);

            assert!(res, "from = {from:?}, dir = {dir:?}")
        })
    }

    #[test]
    fn view_cone_cannot_see_outside_radius() {
        all_directions(|dir| {
            let from = random_coordinate();
            let cone = ViewCone {
                from,
                left_dir: dir,
                right_dir: dir,
                radius: 1,
            };

            let point = cone.from + dir + dir;

            let res = cone.can_see(point, |_| false);

            assert!(!res, "from = {from:?}, dir = {dir:?}")
        })
    }

    #[test]
    fn view_cone_cannot_see_past_obstacle() {
        all_directions(|dir| {
            let from = random_coordinate();
            let cone = ViewCone {
                from,
                left_dir: dir,
                right_dir: dir,
                radius: 2,
            };

            let point = cone.from + dir + dir;

            let obstacle = cone.from + dir;

            let res = cone.can_see(point, move |x| x == obstacle);

            assert!(!res, "from = {from:?}, dir = {dir:?}")
        })
    }
}
