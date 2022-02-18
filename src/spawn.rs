use crate::ai::*;
use crate::domain::common::*;
use crate::domain::turn_queue::TurnQueue;
use crate::intention::PlayerControlled;
use crate::map::*;
use crate::Player;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::*;
use hex2d::*;
use rand::prelude::*;

#[derive(Bundle)]
struct ActorBundle {
    #[bundle]
    shape: ShapeBundle,

    facing: Facing,
    pos: HexPos,
    actor: Actor,
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    actor: ActorBundle,

    player_controlled: PlayerControlled,
    player: Player,
}

#[derive(Bundle)]
struct AiBundle {
    #[bundle]
    actor: ActorBundle,

    ai: AIBehaviour,
}

pub fn spawn_map_entities(
    commands: &mut Commands,
    turn_queue: &mut TurnQueue,
    map: &Map,
) -> Entity {
    fn make_hex_tile(coord: Coordinate) -> RegularPolygon {
        let (x, y) = coord.to_pixel(HEX_SPACING);
        RegularPolygon {
            sides: 6,
            feature: RegularPolygonFeature::Radius(40.0),
            center: Vec2::new(x, -y),
        }
    }
    let map_entity = commands
        .spawn()
        .with_children(|parent| {
            for (&c, cell) in map.cells.iter() {
                let color = match cell.terrain {
                    Terrain::Floor => Color::OLIVE,
                    Terrain::Wall => Color::MIDNIGHT_BLUE,
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
                    .insert(HexPos(c))
                    .insert(MapTile {
                        terrain: cell.terrain,
                    });
            }
        })
        .id();

    spawn_player(commands, turn_queue, map.player_start);

    for c in map
        .cells
        .iter()
        .filter(|(c, cell)| c.distance(map.player_start) > 2 && cell.terrain == Terrain::Floor)
        .map(|(c, _)| c)
        .choose_multiple(&mut thread_rng(), 3)
    {
        spawn_enemy(commands, turn_queue, *c);
    }

    map_entity
}

pub fn spawn_player(
    commands: &mut Commands,
    turn_queue: &mut TurnQueue,
    pos: Coordinate,
) -> Entity {
    let player = commands
        .spawn_bundle(new_player(pos))
        .with_children(|player| {
            player.spawn_bundle(direction_indicator());
        })
        .id();

    turn_queue.enqueue(player);

    player
}

fn new_player(coord: Coordinate) -> PlayerBundle {
    let facing = Facing::default();
    let pos = HexPos(coord);
    let shape = GeometryBuilder::build_as(
        &Circle {
            radius: 30.0,
            center: Vec2::new(0.0, 0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::WHITE),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        Transform::default(),
    );

    let actor = Actor {
        actions_per_turn: 2,
        actions_remaining: 2,
    };

    PlayerBundle {
        actor: ActorBundle {
            facing,
            pos,
            shape,
            actor,
        },

        player_controlled: PlayerControlled,
        player: Player,
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    turn_queue: &mut TurnQueue,
    coordinate: Coordinate,
) -> Entity {
    let enemy = commands
        .spawn_bundle(new_enemy(coordinate))
        .with_children(|parent| {
            parent.spawn_bundle(direction_indicator());
        })
        .id();

    turn_queue.enqueue(enemy);

    enemy
}

fn direction_indicator() -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Polygon {
            points: vec![
                Vec2::new(-15.0, 30.0),
                Vec2::new(0.0, 45.0),
                Vec2::new(15.0, 30.0),
            ],
            closed: true,
        },
        DrawMode::Fill(FillMode::color(Color::YELLOW)),
        Transform::default(),
    )
}

fn new_enemy(coord: Coordinate) -> AiBundle {
    let direction = HexDirection::all().choose(&mut thread_rng()).unwrap();
    let facing = Facing(*direction);
    let pos = HexPos(coord);
    let shape = GeometryBuilder::build_as(
        &Circle {
            radius: 30.0,
            center: Vec2::new(0.0, 0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        Transform::default(),
    );

    let actor = Actor {
        actions_per_turn: 2,
        actions_remaining: 2,
    };

    AiBundle {
        actor: ActorBundle {
            facing,
            pos,
            shape,
            actor,
        },
        ai: AIBehaviour::Wandering,
    }
}
