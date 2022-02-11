use std::iter;

use bevy::prelude::*;

use bevy_easings::EasingsPlugin;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};
use hex2d::*;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

pub mod action_event;
pub mod animation;
pub mod common;
pub mod hex_map;
pub mod intention;
pub mod turn_queue;

use action_event::*;
use animation::*;
use common::*;
use hex_map::HexMap;
use intention::*;
use turn_queue::*;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_event::<IntentionEvent>()
            .add_event::<ActionEvent>()
            .init_resource::<TurnQueue>()
            .add_startup_system(setup)
            .add_system(ingame_keyboard_input.label("produce_intention"))
            .add_system(generate_ai_intentions.label("produce_intention"))
            .add_system(
                process_intention
                    .label("process_intention")
                    .after("produce_intention"),
            )
            .add_system(
                process_action_events
                    .label("process_action_events")
                    .after("process_intention"),
            )
            .add_system(
                cycle_turn_queue
                    .label("process_actions")
                    .after("process_action_events"),
            )
            .add_system(animate_movement.after("process_actions"))
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

fn setup(mut commands: Commands, mut turn_queue: ResMut<TurnQueue>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // spawn map
    let center: Coordinate<i32> = Coordinate::new(0, 0);
    let tiles = (1..5)
        .flat_map(|i| center.ring_iter(i, Spin::CW(XY)))
        .chain(iter::once(center))
        .map(|x| (x, Terrain::random()));
    let map = HexMap::from_iter(tiles);

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
                    &make_hex(c),
                    draw_mode,
                    Transform::default(),
                ))
                .insert(t);
        }
    });

    // spawn player
    let player = commands
        .spawn_bundle(ActorBundle::new_player(Coordinate::new(0, 0)))
        .with_children(|player| {
            player.spawn_bundle(direction_indicator());
        })
        .id();

    turn_queue.queue.push_back(player);

    spawn_enemy(&mut commands, &mut turn_queue, Coordinate::new(2, 2));
    spawn_enemy(&mut commands, &mut turn_queue, Coordinate::new(-2, -2));
}

fn spawn_enemy(commands: &mut Commands, turn_queue: &mut TurnQueue, coordinate: Coordinate) {
    let enemy = commands
        .spawn_bundle(ActorBundle::new_enemy(coordinate))
        .with_children(|parent| {
            parent.spawn_bundle(direction_indicator());
        })
        .id();

    turn_queue.queue.push_back(enemy);
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

#[derive(Bundle)]
struct ActorBundle {
    facing: Facing,
    pos: HexPos,
    actor: Actor,

    #[bundle]
    shape: ShapeBundle,
}

impl ActorBundle {
    fn new_player(coord: Coordinate) -> ActorBundle {
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
            Transform::default().with_translation(pos.as_translation(HEX_SPACING)),
        );

        let actor = Actor {
            control_source: ControlSource::Player,
            actions_per_turn: 2,
            actions_remaining: 2,
        };

        ActorBundle {
            facing,
            pos,
            shape,
            actor,
        }
    }

    fn new_enemy(coord: Coordinate) -> ActorBundle {
        let facing = Facing::default();
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
            Transform::default().with_translation(pos.as_translation(HEX_SPACING)),
        );

        let actor = Actor {
            control_source: ControlSource::AI,
            actions_per_turn: 1,
            actions_remaining: 1,
        };

        ActorBundle {
            facing,
            pos,
            shape,
            actor,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Component)]
enum Terrain {
    Grass,
    Water,
}

impl Terrain {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        *[Terrain::Grass, Terrain::Water].choose(&mut rng).unwrap()
    }
}

fn make_hex(coord: Coordinate) -> RegularPolygon {
    let (x, y) = coord.to_pixel(HEX_SPACING);
    RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(40.0),
        center: Vec2::new(x, y),
    }
}
