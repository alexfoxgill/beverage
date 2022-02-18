use bevy::prelude::*;

use bevy_easings::EasingsPlugin;
use bevy_prototype_lyon::prelude::*;
use domain::turn_queue::{TurnQueue, TurnQueuePlugin};
use wasm_bindgen::prelude::*;

pub mod ai;
pub mod animation;
pub mod component_index;
pub mod domain;
pub mod intention;
pub mod map;
pub mod spawn;
pub mod turn_engine;

use ai::*;
use animation::*;
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
            .add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct Player;
fn setup(mut commands: Commands, mut turn_queue: ResMut<TurnQueue>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let map = SmallHex::generate_map();

    spawn_map_entities(&mut commands, &mut turn_queue, &map);
}
