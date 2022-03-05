use bevy::prelude::*;

use bevy_easings::EasingsPlugin;
use bevy_prototype_lyon::prelude::*;
use domain::turn_queue::TurnQueuePlugin;
use render::GameRenderPlugin;
use scenario::{ArenaScenario, CaveScenario};
use ui::UIPlugin;

pub mod ai;
pub mod camera;
pub mod component_index;
pub mod domain;
pub mod intention;
pub mod map;
pub mod maths;
pub mod pathfinding;
pub mod render;
pub mod scenario;
pub mod spawn;
pub mod turn_engine;
pub mod ui;

use ai::*;
use camera::*;
use domain::*;
use intention::*;
use map::*;
use serde::*;
use turn_engine::*;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub enum Scenario {
    Arena,
    Cave,
}

#[derive(Deserialize)]
pub struct RunParams {
    scenario: Scenario,
}

impl Default for RunParams {
    fn default() -> Self {
        Self {
            scenario: Scenario::Arena,
        }
    }
}

#[wasm_bindgen]
pub fn run_js(js: &JsValue) {
    let params = js.into_serde();
    match params {
        Ok(params) => run(params),
        Err(e) => {
            eprintln!("{e}");
            run(Default::default())
        }
    }
}

pub fn run(params: RunParams) {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(GamePlugin);

    match params.scenario {
        Scenario::Arena => app.add_plugin(ArenaScenario),
        Scenario::Cave => app.add_plugin(CaveScenario),
    };

    app.run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(TurnEnginePlugin)
            .add_plugin(TurnQueuePlugin)
            .add_plugin(DomainPlugin)
            .add_plugin(AiPlugin)
            .add_plugin(IntentionPlugin)
            .add_plugin(GameRenderPlugin)
            .add_plugin(UIPlugin)
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

#[derive(Component)]
pub struct Player;
