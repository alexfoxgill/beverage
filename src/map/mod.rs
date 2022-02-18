use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

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
    Floor,
    Impassable,
}

#[derive(Component)]
pub struct MapTile {
    pub terrain: Terrain,
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
    fn generate_map(&self) -> Map;
}

pub struct BasicHex {
    radius: usize,
}

impl BasicHex {
    pub fn new(radius: usize) -> BasicHex {
        BasicHex { radius }
    }
}

impl MapGenerator for BasicHex {
    fn generate_map(&self) -> Map {
        let mut cells = floor_hex(self.radius);
        surround_wall(&mut cells);

        if let Some(cell) = cells.get_mut(&Coordinate::new(-2, -2)) {
            cell.enemy = Some(HexDirection::YZ);
        }
        if let Some(cell) = cells.get_mut(&Coordinate::new(2, 2)) {
            cell.enemy = Some(HexDirection::ZY);
        }

        Map {
            cells,
            player_start: Coordinate::new(0, -(self.radius as i32 - 2)),
            goal: Coordinate::new(0, 3),
        }
    }
}

fn floor_hex(radius: usize) -> HashMap<Coordinate, MapCell> {
    Coordinate::new(0, 0)
        .range_iter(radius as i32)
        .map(|x| {
            (
                x,
                MapCell {
                    terrain: Terrain::Floor,
                    enemy: None,
                },
            )
        })
        .collect()
}

fn surround_wall(map: &mut HashMap<Coordinate, MapCell>) {
    let mut walls = HashSet::<Coordinate>::default();

    for (pos, cell) in map.iter() {
        if cell.terrain == Terrain::Floor {
            for n in pos.neighbors() {
                if !map.contains_key(&n) {
                    walls.insert(n);
                }
            }
        }
    }

    for wall in walls.iter() {
        map.insert(
            *wall,
            MapCell {
                terrain: Terrain::Impassable,
                enemy: None,
            },
        );
    }
}

fn random_noise<R: Rng>(
    rng: &mut R,
    coordinates: impl Iterator<Item = Coordinate>,
) -> HashMap<Coordinate, MapCell> {
    coordinates
        .map(|c| {
            (
                c,
                MapCell {
                    terrain: if rng.gen::<bool>() {
                        Terrain::Floor
                    } else {
                        Terrain::Impassable
                    },
                    enemy: None,
                },
            )
        })
        .collect::<HashMap<_, _>>()
}

pub struct CellularAutomata;

impl MapGenerator for CellularAutomata {
    fn generate_map(&self) -> Map {
        let mut rng = thread_rng();

        let cells = random_noise(&mut rng, Coordinate::new(0, 0).range_iter(5));

        Map {
            cells,
            player_start: Coordinate::new(0, 0),
            goal: Coordinate::new(0, 0),
        }
    }
}
