use bevy::prelude::*;

use crate::turn_engine::TurnSystems;

pub mod backstep;
pub mod end_turn;
pub mod rotate;
pub mod step;
pub mod strike;

pub struct DomainActionsPlugin;

impl Plugin for DomainActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_action_handler(backstep::handler.system());
    systems.register_action_handler(end_turn::handler.system());
    systems.register_action_handler(rotate::handler.system());
    systems.register_action_handler(step::handler.system());
    systems.register_action_handler(strike::handler.system());
}
