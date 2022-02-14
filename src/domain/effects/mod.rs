use bevy::prelude::*;

use self::{
    end_turn::EndTurnEffectPlugin, energy_cost::EnergyCostEffectPlugin, face::FaceEffectPlugin,
    kill::KillEffectPlugin, move_entity::MoveEffectPlugin,
};

pub mod end_turn;
pub mod energy_cost;
pub mod face;
pub mod kill;
pub mod move_entity;

pub struct DomainEffectsPlugin;

impl Plugin for DomainEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EndTurnEffectPlugin)
            .add_plugin(EnergyCostEffectPlugin)
            .add_plugin(FaceEffectPlugin)
            .add_plugin(KillEffectPlugin)
            .add_plugin(MoveEffectPlugin);
    }
}
