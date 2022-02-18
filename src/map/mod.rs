use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use hex2d::{Direction as HexDirection, *};
use itertools::iterate;
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MapCell {
    pub terrain: Terrain,
    pub enemy: Option<HexDirection>,
}

impl MapCell {
    fn floor() -> MapCell {
        MapCell {
            terrain: Terrain::Floor,
            enemy: None,
        }
    }
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
        }
    }
}

fn floor_hex(radius: usize) -> HashMap<Coordinate, MapCell> {
    Coordinate::new(0, 0)
        .range_iter(radius as i32)
        .map(|x| (x, MapCell::floor()))
        .collect()
}

fn surround_wall(map: &mut HashMap<Coordinate, MapCell>) {
    let walls: HashSet<_> = map
        .iter()
        .filter(|(_, c)| c.terrain == Terrain::Floor)
        .flat_map(|(pos, _)| pos.neighbors())
        .filter(|n| !map.contains_key(n))
        .collect();

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

fn random_noise(coordinates: impl Iterator<Item = Coordinate>) -> HashMap<Coordinate, MapCell> {
    let mut rng = thread_rng();
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
        .collect()
}

pub struct CellularAutomata {
    radius: usize,
    iterations: usize,
    rule: fn(usize) -> Terrain,
}

impl CellularAutomata {
    pub fn example() -> CellularAutomata {
        CellularAutomata::new(8, 10, |x| match x {
            0 | 1 | 5 => Terrain::Wall,
            _ => Terrain::Floor,
        })
    }

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

fn choose_random(cells: &HashMap<Coordinate, MapCell>) -> Coordinate {
    let mut rng = thread_rng();
    *cells
        .iter()
        .filter(|(_, c)| c.terrain == Terrain::Floor)
        .map(|(c, _)| c)
        .choose(&mut rng)
        .unwrap()
}

impl MapGenerator for CellularAutomata {
    fn generate_map(&self) -> Map {
        let mut cells = random_noise(Coordinate::new(0, 0).range_iter(self.radius as i32));

        self.process(&mut cells);
        surround_wall(&mut cells);

        let player_start = choose_random(&cells);

        Map {
            cells,
            player_start,
        }
    }
}

pub struct DrunkardsWalk {
    distance: usize,
    limit: usize,
}

impl DrunkardsWalk {
    pub fn example() -> DrunkardsWalk {
        DrunkardsWalk {
            distance: 40,
            limit: 200,
        }
    }

    fn gen_path(&self, start: Coordinate) -> impl Iterator<Item = Coordinate> {
        let mut rng = thread_rng();
        iterate(start, move |x| {
            let dir = HexDirection::all().choose(&mut rng).unwrap();
            *x + *dir
        })
        .take(self.distance)
    }

    fn carve_path(&self, start: Coordinate, map: &mut HashMap<Coordinate, MapCell>) {
        let path = self.gen_path(start).map(|c| (c, MapCell::floor()));
        map.extend(path);
    }
}

impl MapGenerator for DrunkardsWalk {
    fn generate_map(&self) -> Map {
        let mut path_start = Coordinate::new(0, 0);

        let mut cells = HashMap::<Coordinate, MapCell>::default();

        loop {
            self.carve_path(path_start, &mut cells);
            if cells.len() > self.limit {
                break;
            }
            path_start = choose_random(&cells);
        }

        surround_wall(&mut cells);

        let player_start = choose_random(&cells);

        Map {
            cells,
            player_start,
        }
    }
}
