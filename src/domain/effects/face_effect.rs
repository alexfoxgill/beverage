use bevy::prelude::*;

use crate::common::{Facing, HexDirection};

use crate::turn_engine::effects::{Effect, EffectEvent};
use crate::turn_engine::{Handled, TurnSchedules};

#[derive(Debug, Clone)]
pub struct FaceEffect {
    entity: Entity,
    face: HexDirection,
}

impl FaceEffect {
    pub fn event(entity: Entity, face: HexDirection) -> EffectEvent {
        EffectEvent(Box::new(FaceEffect { entity, face }))
    }
}

impl Effect for FaceEffect {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct FaceEffectPlugin;

impl Plugin for FaceEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_effect_handler::<FaceEffect>(schedule)
}

fn handler(mut facings: Query<&mut Facing>, effect: Res<Handled<FaceEffect>>) {
    if let Ok(mut facing) = facings.get_mut(effect.0.entity) {
        facing.0 = effect.0.face;
    }
}