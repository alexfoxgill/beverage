use crate::ai::*;
use crate::domain::common::*;
use crate::domain::turn_queue::TurnQueue;
use crate::domain::vision::Vision;
use crate::domain::vision::VisionType;
use crate::intention::PlayerControlled;
use crate::map::*;
use crate::maths::RADIANS_120DEG;
use crate::render::actor::render_enemy;
use crate::render::actor::render_player;
use crate::render::map::tile_render_bundle;
use crate::render::player_vision::PlayerVisibility;
use crate::Player;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
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

    vision: Vision,
    player_controlled: PlayerControlled,
    player: Player,
}

#[derive(Bundle)]
struct AiBundle {
    #[bundle]
    actor: ActorBundle,

    ai: AIBehaviour,
    player_vis: PlayerVisibility,
}

#[derive(Bundle)]
struct MapTileBundle {
    #[bundle]
    shape: ShapeBundle,

    pos: HexPos,
    tile: MapTile,
    player_vis: PlayerVisibility,
}

pub fn spawn_map_entities(
    commands: &mut Commands,
    turn_queue: &mut TurnQueue,
    map: &Map,
) -> Entity {
    let map_entity = commands
        .spawn()
        .with_children(|parent| {
            for (&c, cell) in map.cells.iter() {
                let tile = MapTile {
                    terrain: cell.terrain,
                };
                let shape = tile_render_bundle(c, &tile);
                parent.spawn_bundle(MapTileBundle {
                    shape,
                    pos: HexPos(c),
                    tile,
                    player_vis: PlayerVisibility::new_persistent(),
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
        spawn_enemy(commands, turn_queue, *c, AIBehaviour::Wandering);
    }

    map_entity
}

pub fn spawn_player(
    commands: &mut Commands,
    turn_queue: &mut TurnQueue,
    pos: Coordinate,
) -> Entity {
    let player = commands.spawn_bundle(new_player(pos)).id();

    turn_queue.enqueue(player);

    player
}

fn new_player(coord: Coordinate) -> PlayerBundle {
    let facing = Facing::default();
    let pos = HexPos(coord);
    let shape = render_player();

    let actor = Actor {
        actions_per_turn: 2,
        actions_remaining: 2,
    };

    let vision = VisionType::Radial(5)
        .and(VisionType::Conical(RADIANS_120DEG))
        .or(VisionType::Radial(1))
        .and(VisionType::Obstructable);

    PlayerBundle {
        actor: ActorBundle {
            facing,
            pos,
            shape,
            actor,
        },

        vision: Vision::new(vision),
        player_controlled: PlayerControlled,
        player: Player,
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    turn_queue: &mut TurnQueue,
    coordinate: Coordinate,
    ai: AIBehaviour,
) -> Entity {
    let enemy = commands.spawn_bundle(new_enemy(coordinate, ai)).id();

    turn_queue.enqueue(enemy);

    enemy
}

fn new_enemy(coord: Coordinate, ai: AIBehaviour) -> AiBundle {
    let direction = HexDirection::all().choose(&mut thread_rng()).unwrap();
    let facing = Facing(*direction);
    let pos = HexPos(coord);
    let shape = render_enemy();

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
        ai,
        player_vis: PlayerVisibility::new_transient(),
    }
}
