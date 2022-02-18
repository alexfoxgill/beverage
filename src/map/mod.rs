use std::iter;

use bevy::prelude::*;

use hex2d::{Direction as HexDirection, *};
use rand::prelude::*;

mod hex_map;

use crate::{component_index::ComponentIndex, domain::common::HexPos};

use hex_map::HexMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ComponentIndex::<HexPos>::plugin());
    }
}

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

pub struct Map {
    pub cells: HexMap<MapCell>,
    pub player_start: Coordinate,
    pub goal: Coordinate,
}

pub struct MapCell {
    pub terrain: Terrain,
    pub enemy: Option<HexDirection>
}

pub trait MapGenerator {
    fn generate_map() -> Map;
}

pub struct SmallHex;

impl MapGenerator for SmallHex {
    fn generate_map() -> Map {
        let center: Coordinate<i32> = Coordinate::new(0, 0);
        let tiles = (1..5)
            .flat_map(|i| center.ring_iter(i, Spin::CW(XY)))
            .chain(iter::once(center))
            .map(|x| {
                (
                    x,
                    MapCell {
                        terrain: Terrain::random(),
                        enemy: None
                    },
                )
            });
        let cells = HexMap::from_iter(tiles);

        Map {
            cells,
            player_start: Coordinate::new(0, 0)
                + HexDirection::ZY
                + HexDirection::ZY
                + HexDirection::ZY,
            goal: Coordinate::new(0, 0) + HexDirection::YZ + HexDirection::YZ + HexDirection::YZ,
        }
    }
}
