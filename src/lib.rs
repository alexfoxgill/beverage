use bevy::prelude::*;

use bevy_easings::EasingsPlugin;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};
use hex2d::*;
use turn_queue::{TurnQueue, TurnQueuePlugin};
use wasm_bindgen::prelude::*;

pub mod ai;
pub mod animation;
pub mod domain;
pub mod hex_map;
pub mod intention;
pub mod map;
pub mod turn_engine;
pub mod turn_queue;

use ai::*;
use animation::*;
use domain::common::*;
use domain::*;
use intention::*;
use map::*;
use turn_engine::*;

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
            .add_state(AnimatingState::Still)
            .add_plugin(MapPlugin)
            .add_plugin(TurnEnginePlugin)
            .add_plugin(TurnQueuePlugin)
            .add_plugin(DomainPlugin)
            .add_plugin(AiPlugin)
            .add_plugin(IntentionPlugin)
            .add_stage_after(
                TurnExecution,
                "blah",
                SystemStage::parallel()
                    .with_system(animate_movement.label("run_animations"))
                    .with_system(update_animating_state.after("run_animations")),
            )
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_startup_system(setup.after(SpawnMap));
    }
}

fn setup(mut commands: Commands, mut turn_queue: ResMut<TurnQueue>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // spawn player
    let player = commands
        .spawn_bundle(ActorBundle::new_player(Coordinate::new(0, 0)))
        .with_children(|player| {
            player.spawn_bundle(direction_indicator());
        })
        .id();

    turn_queue.enqueue(player);

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

    turn_queue.enqueue(enemy);
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
    #[bundle]
    shape: ShapeBundle,

    facing: Facing,
    pos: HexPos,
    actor: Actor,
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    actor: ActorBundle,

    player: Player,
}

#[derive(Bundle)]
struct AiBundle {
    #[bundle]
    actor: ActorBundle,

    ai: AIBehaviour,
}

impl ActorBundle {
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

            player: Player,
        }
    }

    fn new_enemy(coord: Coordinate) -> AiBundle {
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
            Transform::default(),
        );

        let actor = Actor {
            actions_per_turn: 1,
            actions_remaining: 1,
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
}
