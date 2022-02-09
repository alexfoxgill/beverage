use std::{
    collections::{hash_map, VecDeque},
    iter,
    time::Duration,
};

use bevy::{prelude::*, utils::HashMap};

use bevy_easings::{Ease, EaseFunction, EasingType, EasingsPlugin};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};
use hex2d::{Direction as HexDirection, *};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

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
                process_actions
                    .label("process_actions")
                    .after("process_intention"),
            )
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HexMap<T> {
    map: HashMap<Coordinate, T>,
}

impl<T> HexMap<T> {
    pub fn insert(&mut self, coord: Coordinate, value: T) -> Option<T> {
        self.map.insert(coord, value)
    }

    pub fn iter(&self) -> hash_map::Iter<'_, Coordinate, T> {
        self.map.iter()
    }
}

impl<T> Default for HexMap<T> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<T> FromIterator<(Coordinate, T)> for HexMap<T> {
    fn from_iter<Iter: IntoIterator<Item = (Coordinate, T)>>(iter: Iter) -> Self {
        Self {
            map: FromIterator::from_iter(iter),
        }
    }
}

impl<T> Extend<(Coordinate, T)> for HexMap<T> {
    fn extend<Iter: IntoIterator<Item = (Coordinate, T)>>(&mut self, iter: Iter) {
        self.map.extend(iter)
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

    commands.spawn().insert(Map).with_children(|parent| {
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

#[derive(Default)]
struct TurnQueue {
    queue: VecDeque<Entity>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ControlSource {
    Player,
    AI,
}

#[derive(Component)]
struct Actor {
    control_source: ControlSource,
    actions_per_turn: i32,
    actions_remaining: i32,
}

struct IntentionEvent(Entity, Intention);

enum Intention {
    Rotate(Angle),
    Move(Angle),
    Attack(Angle),
    EndTurn,
}

fn ingame_keyboard_input(
    keys: Res<Input<KeyCode>>,
    actors: Query<&Actor>,
    turn_queue: Res<TurnQueue>,
    mut ev_intention: EventWriter<IntentionEvent>,
) {
    if let Some(&entity) = turn_queue.queue.front() {
        if let Ok(actor) = actors.get(entity) {
            if actor.control_source == ControlSource::Player {
                if keys.just_pressed(KeyCode::Left) {
                    ev_intention.send(IntentionEvent(entity, Intention::Rotate(Angle::Left)));
                }
                if keys.just_pressed(KeyCode::Right) {
                    ev_intention.send(IntentionEvent(entity, Intention::Rotate(Angle::Right)));
                }
                if keys.just_pressed(KeyCode::Up) {
                    ev_intention.send(IntentionEvent(entity, Intention::Move(Angle::Forward)));
                }
                if keys.just_pressed(KeyCode::Down) {
                    ev_intention.send(IntentionEvent(entity, Intention::Move(Angle::Back)));
                }
                if keys.just_pressed(KeyCode::E) {
                    ev_intention.send(IntentionEvent(entity, Intention::EndTurn));
                }
                if keys.just_pressed(KeyCode::Space) {
                    ev_intention.send(IntentionEvent(entity, Intention::Attack(Angle::Forward)));
                }
            }
        }
    }
}

fn generate_ai_intentions(
    actors: Query<(&HexPos, &Facing, &Actor)>,
    turn_queue: Res<TurnQueue>,
    mut ev_intention: EventWriter<IntentionEvent>,
) {
    if let Some(&entity) = turn_queue.queue.front() {
        if let Ok((_, _, actor)) = actors.get(entity) {
            if actor.control_source == ControlSource::AI {
                let rotation = Angle::from_int(rand::thread_rng().gen_range(1..=6));

                ev_intention.send(IntentionEvent(entity, Intention::Rotate(rotation)));
                ev_intention.send(IntentionEvent(entity, Intention::Move(Angle::Forward)));
            }
        }
    }
}

fn process_intention(
    mut commands: Commands,
    mut actors: Query<(&mut Facing, &mut HexPos, &mut Actor, &Transform, Entity)>,
    mut ev_intention: EventReader<IntentionEvent>,
) {
    for IntentionEvent(entity, intention) in ev_intention.iter() {
        let (mut facing, mut pos, mut actor, transform, _) = actors.get_mut(*entity).unwrap();
        let init_transform = Transform {
            rotation: facing.as_rotation(),
            translation: pos.as_translation(Spacing::FlatTop(40.0)),
            ..Default::default()
        };

        match intention {
            Intention::Rotate(angle) => {
                facing.rotate(*angle);
                commands.entity(*entity).insert(transform.ease_to(
                    init_transform.with_rotation(facing.as_rotation()),
                    EaseFunction::QuadraticInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(50),
                    },
                ));
            }
            Intention::Move(angle) => {
                actor.actions_remaining -= 1;
                pos.move_facing(facing.rotated(*angle));
                commands.entity(*entity).insert(transform.ease_to(
                    init_transform.with_translation(pos.as_translation(Spacing::FlatTop(40.0))),
                    EaseFunction::QuadraticInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(200),
                    },
                ));
            }
            Intention::EndTurn => {
                actor.actions_remaining = 0;
            }
            Intention::Attack(angle) => {
                let direction = facing.rotated(*angle);
                let coord_to_attack = pos.get_facing(direction);
                for (_, pos, _, _, e) in actors.iter() {
                    if pos.0 == coord_to_attack {
                        commands.entity(e).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn process_actions(mut query: Query<&mut Actor>, mut turn_queue: ResMut<TurnQueue>) {
    if let Some(&entity) = turn_queue.queue.front() {
        if let Ok(mut actor) = query.get_mut(entity) {
            if actor.actions_remaining == 0 {
                actor.actions_remaining = actor.actions_per_turn;
                turn_queue.queue.pop_front();
                turn_queue.queue.push_back(entity);
            }
        } else {
            turn_queue.queue.pop_front();
        }
    }
}

#[derive(Component)]
struct Map;

#[derive(Component)]
struct Facing(HexDirection);

impl Facing {
    pub fn rotated(&self, angle: Angle) -> HexDirection {
        self.0 + angle
    }

    pub fn rotate(&mut self, angle: Angle) {
        self.0 = self.rotated(angle);
    }

    pub fn as_rotation(&self) -> Quat {
        // the z-axis points towards the camera, so rotate a negative amount
        Quat::from_rotation_z(-self.0.to_radians_flat::<f32>())
    }
}

#[derive(Component)]
pub struct HexPos(Coordinate);

impl HexPos {
    pub fn get_facing(&self, dir: HexDirection) -> Coordinate {
        self.0 + dir
    }

    pub fn move_facing(&mut self, dir: HexDirection) {
        self.0 = self.get_facing(dir);
    }

    pub fn as_translation(&self, spacing: Spacing) -> Vec3 {
        let (x, y) = self.0.to_pixel(spacing);
        // the y-axis points upward, so invert it
        Vec3::new(x, -y, 0.0)
    }
}

impl Default for HexPos {
    fn default() -> Self {
        Self(Coordinate::new(0, 0))
    }
}

#[derive(Component)]
struct DirectionIndicator;

impl Default for Facing {
    fn default() -> Self {
        Self(HexDirection::YZ)
    }
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
            Transform::default().with_translation(pos.as_translation(Spacing::FlatTop(40.0))),
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
            Transform::default().with_translation(pos.as_translation(Spacing::FlatTop(40.0))),
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
    let (x, y) = coord.to_pixel(Spacing::FlatTop(40.0));
    RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(40.0),
        center: Vec2::new(x, y),
    }
}
