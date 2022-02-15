use bevy::prelude::*;

use crate::turn_engine::TurnSystems;

pub mod end_turn;
pub mod energy_cost;
pub mod face;
pub mod kill;
pub mod move_entity;

pub struct DomainEffectsPlugin;

impl Plugin for DomainEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_effect_handler(end_turn::handler.system());
    systems.register_effect_handler(energy_cost::handler.system());
    systems.register_effect_handler(face::handler.system());
    systems.register_effect_handler(kill::handler.system());
    systems.register_effect_handler(move_entity::handler.system());
}
