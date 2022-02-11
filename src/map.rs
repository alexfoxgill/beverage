use std::iter;

use bevy::prelude::*;

use bevy_prototype_lyon::prelude::*;
use hex2d::*;
use rand::prelude::*;

use crate::{common::HEX_SPACING, hex_map::HexMap};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Terrain {
    Grass,
    Water,
}

#[derive(Component)]
pub struct MapTile {
    pub terrain: Terrain,
}

impl Terrain {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        *[Terrain::Grass, Terrain::Water].choose(&mut rng).unwrap()
    }
}

fn make_hex_tile(coord: Coordinate) -> RegularPolygon {
    let (x, y) = coord.to_pixel(HEX_SPACING);
    RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(40.0),
        center: Vec2::new(x, y),
    }
}

pub fn spawn_map(commands: &mut Commands) {
    let map = generate_map();

    commands.spawn().with_children(|parent| {
        for (&c, &t) in map.iter() {
            let color = match t {
                Terrain::Grass => Color::OLIVE,
                Terrain::Water => Color::TEAL,
            };
            let draw_mode = DrawMode::Outlined {
                fill_mode: FillMode::color(color),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            };
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &make_hex_tile(c),
                    draw_mode,
                    Transform::default(),
                ))
                .insert(MapTile { terrain: t });
        }
    });
}

pub fn generate_map() -> HexMap<Terrain> {
    let center: Coordinate<i32> = Coordinate::new(0, 0);
    let tiles = (1..5)
        .flat_map(|i| center.ring_iter(i, Spin::CW(XY)))
        .chain(iter::once(center))
        .map(|x| (x, Terrain::random()));
    HexMap::from_iter(tiles)
}
