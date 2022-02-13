use bevy::prelude::*;
use hex2d::Coordinate;

use crate::{
    common::HexPos,
    turn_engine::{effects::Effect, Handled, TurnSchedules},
};

#[derive(Debug, Clone)]
pub struct MoveEffect {
    entity: Entity,
    to: Coordinate,
}

impl MoveEffect {
    pub fn new(entity: Entity, to: Coordinate) -> MoveEffect {
        MoveEffect { entity, to }
    }
}

impl Effect for MoveEffect {
    fn insert_handled(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct MoveEffectPlugin;

impl Plugin for MoveEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_effect_handler::<MoveEffect>(schedule)
}

fn handler(mut positions: Query<&mut HexPos>, effect: Res<Handled<MoveEffect>>) {
    if let Ok(mut pos) = positions.get_mut(effect.0.entity) {
        pos.0 = effect.0.to;
    }
}
