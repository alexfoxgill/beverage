use bevy::prelude::*;

use self::{
    attack::AttackActionPlugin, backstep::BackstepActionPlugin,
    end_turn::EndTurnActionPlugin, rotate::RotateActionPlugin,
    step::StepActionPlugin,
};

pub mod attack;
pub mod backstep;
pub mod end_turn;
pub mod rotate;
pub mod step;

pub struct DomainActionsPlugin;

impl Plugin for DomainActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AttackActionPlugin)
            .add_plugin(BackstepActionPlugin)
            .add_plugin(EndTurnActionPlugin)
            .add_plugin(StepActionPlugin)
            .add_plugin(RotateActionPlugin);
    }
}
