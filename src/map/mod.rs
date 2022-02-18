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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Terrain {
    Floor,
    Wall,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
                terrain: Terrain::Wall,
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
                        Terrain::Wall
                    },
                    enemy: None,
                },
            )
        })
        .collect::<HashMap<_, _>>()
}

pub struct CellularAutomata {
    radius: usize,
    iterations: usize,
    rule: fn(usize) -> Terrain,
}

impl CellularAutomata {
    pub fn new(radius: usize, iterations: usize, rule: fn(usize) -> Terrain) -> CellularAutomata {
        CellularAutomata {
            radius,
            iterations,
            rule,
        }
    }

    fn step(&self, cells: &mut HashMap<Coordinate, MapCell>) {
        *cells = cells
            .iter()
            .map(|(&pos, &cell)| {
                let neighbor_count = pos
                    .neighbors()
                    .iter()
                    .map(|c| cells.get(&c))
                    .flatten()
                    .filter(|c| c.terrain == Terrain::Floor)
                    .count();
                (
                    pos,
                    MapCell {
                        terrain: (self.rule)(neighbor_count),
                        ..cell
                    },
                )
            })
            .collect();
    }

    fn process(&self, cells: &mut HashMap<Coordinate, MapCell>) {
        for _ in 0..self.iterations {
            self.step(cells);
        }
    }
}

impl MapGenerator for CellularAutomata {
    fn generate_map(&self) -> Map {
        let mut rng = thread_rng();

        let mut cells = random_noise(
            &mut rng,
            Coordinate::new(0, 0).range_iter(self.radius as i32),
        );

        self.process(&mut cells);

        surround_wall(&mut cells);

        Map {
            cells,
            player_start: Coordinate::new(0, 0),
            goal: Coordinate::new(0, 0),
        }
    }
}
