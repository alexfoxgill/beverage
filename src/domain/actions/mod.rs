use bevy::prelude::*;

use self::{
    attack_action::AttackActionPlugin, backstep_action::BackstepActionPlugin,
    step_action::StepActionPlugin, end_turn_action::EndTurnActionPlugin, rotate_action::RotateActionPlugin,
};

pub mod attack_action;
pub mod backstep_action;
pub mod end_turn_action;
pub mod rotate_action;
pub mod step_action;

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
