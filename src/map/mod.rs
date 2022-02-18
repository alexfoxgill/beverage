use std::iter;

use bevy::{prelude::*, utils::HashMap};

use hex2d::{Direction as HexDirection, *};
use rand::prelude::*;

use crate::{component_index::ComponentIndex, domain::common::HexPos};

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
    pub cells: HashMap<Coordinate, MapCell>,
    pub player_start: Coordinate,
    pub goal: Coordinate,
}

pub struct MapCell {
    pub terrain: Terrain,
    pub enemy: Option<HexDirection>,
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
                        enemy: None,
                    },
                )
            });
        let mut cells = HashMap::from_iter(tiles);

        if let Some(cell) = cells.get_mut(&Coordinate::new(-2, -2)) {
            cell.enemy = Some(HexDirection::YZ);
        }
        if let Some(cell) = cells.get_mut(&Coordinate::new(2, 2)) {
            cell.enemy = Some(HexDirection::ZY);
        }

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
