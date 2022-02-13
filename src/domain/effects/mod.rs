use bevy::prelude::*;

use self::{
    energy_cost_effect::EnergyCostEffectPlugin, face_effect::FaceEffectPlugin,
    kill_effect::KillEffectPlugin, move_effect::MoveEffectPlugin,
};

pub mod energy_cost_effect;
pub mod face_effect;
pub mod kill_effect;
pub mod move_effect;

pub struct DomainEffectsPlugin;

impl Plugin for DomainEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EnergyCostEffectPlugin)
            .add_plugin(FaceEffectPlugin)
            .add_plugin(KillEffectPlugin)
            .add_plugin(MoveEffectPlugin);
    }
}
