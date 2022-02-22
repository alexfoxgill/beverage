use bevy::prelude::*;

use bevy_easings::EasingsPlugin;
use bevy_prototype_lyon::prelude::*;
use domain::turn_queue::{TurnQueue, TurnQueuePlugin};
use player_vision::PlayerVisionPlugin;
use render::{animation::AnimationPlugin, DomainRenderPlugin};
use wasm_bindgen::prelude::*;

pub mod ai;
pub mod camera;
pub mod component_index;
pub mod domain;
pub mod intention;
pub mod map;
pub mod pathfinding;
pub mod player_vision;
pub mod render;
pub mod spawn;
pub mod turn_engine;

use ai::*;
use camera::*;
use domain::*;
use intention::*;
use map::*;
use spawn::*;
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
            .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.4)))
            .add_plugin(CameraPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(TurnEnginePlugin)
            .add_plugin(TurnQueuePlugin)
            .add_plugin(DomainPlugin)
            .add_plugin(AiPlugin)
            .add_plugin(IntentionPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(PlayerVisionPlugin)
            .add_plugin(DomainRenderPlugin)
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, mut turn_queue: ResMut<TurnQueue>) {
    let map = DrunkardsWalk::example().generate_map();

    spawn_map_entities(&mut commands, &mut turn_queue, &map);
}

#[derive(Component)]
pub struct Player;
