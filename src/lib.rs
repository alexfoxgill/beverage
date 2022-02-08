use std::{collections::hash_map, iter, time::Duration};

use bevy::{prelude::*, utils::HashMap};

use bevy_easings::{Ease, EaseFunction, EasingType, EasingsPlugin};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use hex2d::{Direction as HexDirection, *};
use rand::{prelude::SliceRandom, thread_rng};
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum TurnState {
    PlayerTurn,
    EnemyTurn,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_event::<PlayerIntentionEvent>()
            .add_state(TurnState::PlayerTurn)
            .add_startup_system(setup)
            .add_system(keyboard_input.label("input"))
            .add_system(player_movement.label("movement").after("input"))
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

fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

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

    commands
        .spawn_bundle(PlayerBundle::default())
        .insert_bundle(GeometryBuilder::build_as(
            &Circle {
                radius: 30.0,
                center: Vec2::new(0.0, 0.0),
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::default(),
        ))
        .with_children(|player| {
            player.spawn().insert_bundle(GeometryBuilder::build_as(
                &shapes::Polygon {
                    points: vec![
                        Vec2::new(-15.0, 0.0),
                        Vec2::new(0.0, 15.0),
                        Vec2::new(15.0, 0.0),
                    ],
                    closed: true,
                },
                DrawMode::Fill(FillMode::color(Color::YELLOW)),
                Transform::from_translation(Vec3::new(0.0, 30.0, 0.0)),
            ));
        });
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PlayerIntentionEvent {
    Rotate(Angle),
    Move(Angle),
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut ev_intention: EventWriter<PlayerIntentionEvent>) {
    if keys.just_pressed(KeyCode::Left) {
        ev_intention.send(PlayerIntentionEvent::Rotate(Angle::Left));
    }
    if keys.just_pressed(KeyCode::Right) {
        ev_intention.send(PlayerIntentionEvent::Rotate(Angle::Right));
    }
    if keys.just_pressed(KeyCode::Up) {
        ev_intention.send(PlayerIntentionEvent::Move(Angle::Forward));
    }
    if keys.just_pressed(KeyCode::Down) {
        ev_intention.send(PlayerIntentionEvent::Move(Angle::Back));
    }
}

fn player_movement(
    mut commands: Commands,
    mut query: Query<(&mut Facing, &mut HexPos, &Transform, Entity), With<Player>>,
    mut ev_intention: EventReader<PlayerIntentionEvent>,
) {
    let (mut facing, mut pos, transform, entity) = query.single_mut();
    let init_transform = Transform {
        rotation: facing.as_rotation(),
        translation: pos.as_translation(Spacing::FlatTop(40.0)),
        ..Default::default()
    };
    for ev in ev_intention.iter() {
        let next_transform = match ev {
            PlayerIntentionEvent::Rotate(angle) => {
                facing.rotate(*angle);
                transform.ease_to(
                    init_transform.with_rotation(facing.as_rotation()),
                    EaseFunction::QuadraticInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(50),
                    },
                )
            }
            PlayerIntentionEvent::Move(angle) => {
                pos.move_dir(facing.0 + *angle);
                transform.ease_to(
                    init_transform.with_translation(pos.as_translation(Spacing::FlatTop(40.0))),
                    EaseFunction::QuadraticInOut,
                    EasingType::Once {
                        duration: Duration::from_millis(200),
                    },
                )
            }
        };
        commands.entity(entity).insert(next_transform);
    }
}

#[derive(Component)]
struct Map;

#[derive(Component, Default)]
struct Player;

#[derive(Component)]
struct Facing(HexDirection);

impl Facing {
    pub fn rotate(&mut self, angle: Angle) {
        self.0 = self.0 + angle;
    }

    pub fn as_rotation(&self) -> Quat {
        // the z-axis points towards the camera, so rotate a negative amount
        Quat::from_rotation_z(-self.0.to_radians_flat::<f32>())
    }
}

#[derive(Component)]
pub struct HexPos(Coordinate);

impl HexPos {
    pub fn move_dir(&mut self, dir: HexDirection) {
        self.0 = self.0 + dir;
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

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    facing: Facing,
    pos: HexPos,
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
