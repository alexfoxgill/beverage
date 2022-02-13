use bevy::prelude::*;

use self::{actions::ActionPlugin, effects::EffectPlugin};

pub mod actions;
pub mod action_queue;
pub mod effects;

pub struct TurnEnginePlugin;

impl Plugin for TurnEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionPlugin).add_plugin(EffectPlugin);
    }
}
