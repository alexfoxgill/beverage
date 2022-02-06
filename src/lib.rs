use std::{collections::hash_map, iter, ops::Neg};

use bevy::{prelude::*, utils::HashMap};

use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use hex2d::{Direction as HexDirection, *};
use rand::{prelude::SliceRandom, thread_rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_startup_system(setup)
            .add_system(keyboard_input)
            .add_stage_after(
                CoreStage::Update,
                "update_transform",
                SystemStage::single_threaded(),
            )
            .add_system_to_stage("update_transform", hex_transform_sync)
            .add_system_to_stage("update_transform", hex_rotation_sync)
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

fn hex_transform_sync(mut query: Query<(&HexPos, &mut Transform)>) {
    for (&HexPos(pos), mut transform) in query.iter_mut() {
        let (x, y) = pos.to_pixel(Spacing::FlatTop(40.0));
        transform.translation = Vec3::new(x, -y, 0.0);
    }
}

fn hex_rotation_sync(mut query: Query<(&Facing, &mut Transform)>) {
    for (&Facing(dir), mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(-dir.to_radians_flat::<f32>());
    }
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Facing, &mut HexPos), With<Player>>,
) {
    let (mut facing, mut pos) = query.single_mut();
    if keys.just_pressed(KeyCode::Left) {
        facing.rotate_left();
    }
    if keys.just_pressed(KeyCode::Right) {
        facing.rotate_right();
    }
    if keys.just_pressed(KeyCode::Up) {
        pos.move_dir(facing.0);
    }
    if keys.just_pressed(KeyCode::Down) {
        pos.move_dir(-facing.0);
    }
}

#[derive(Component)]
struct Map;

#[derive(Component, Default)]
struct Player;

#[derive(Component)]
struct Facing(HexDirection);

impl Facing {
    pub fn rotate_left(&mut self) {
        self.rotate(Angle::Left);
    }

    pub fn rotate_right(&mut self) {
        self.rotate(Angle::Right);
    }

    pub fn rotate(&mut self, angle: Angle) {
        self.0 = self.0 + angle;
    }
}

#[derive(Component)]
pub struct HexPos(Coordinate);

impl HexPos {
    pub fn move_dir(&mut self, dir: HexDirection) {
        self.0 = self.0 + dir;
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
