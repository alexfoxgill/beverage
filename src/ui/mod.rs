pub mod energy_counter;
pub mod move_list;

use bevy::prelude::*;

use self::{energy_counter::EnergyCounterPlugin, move_list::MoveListPlugin};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_plugin(MoveListPlugin)
            .add_plugin(EnergyCounterPlugin);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
