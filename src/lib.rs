use bevy::{core::FixedTimestep, prelude::*};

use bevy_prototype_lyon::prelude::*;
use hex2d::*;
use wasm_bindgen::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            )
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

#[derive(Component)]
pub struct HexPos(Coordinate);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let center = Coordinate::new(0, 0);
    create_entity(&mut commands, Color::CYAN, center);
    for i in 1..5 {
        for pos in center.ring_iter(i, Spin::CW(XY)) {
            create_entity(&mut commands, Color::AQUAMARINE, pos);
        }
    }
}

fn create_entity(commands: &mut Commands, color: Color, coord: Coordinate) {
    let (x, y) = coord.to_pixel(Spacing::FlatTop(40.0));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 6,
                feature: RegularPolygonFeature::Radius(40.0),
                ..Default::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(color),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform {
                translation: Vec3::new(x, y, 0.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..Default::default()
            },
        ))
        .insert(HexPos(coord));
}
