use bevy::prelude::*;

use self::{actions::DomainActionsPlugin, effects::DomainEffectsPlugin};

pub mod actions;
pub mod common;
pub mod effects;
pub mod turn_queue;
pub mod vision;

pub struct DomainPlugin;

impl Plugin for DomainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DomainActionsPlugin)
            .add_plugin(DomainEffectsPlugin);
    }
}
