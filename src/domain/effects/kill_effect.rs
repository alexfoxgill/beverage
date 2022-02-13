use bevy::prelude::*;

use crate::turn_engine::{
    effects::{Effect, EffectEvent},
    Handled, TurnSchedules,
};

#[derive(Debug, Clone)]
pub struct KillEffect {
    entity: Entity,
}

impl KillEffect {
    pub fn event(entity: Entity) -> EffectEvent {
        EffectEvent(Box::new(KillEffect { entity }))
    }
}

impl Effect for KillEffect {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct KillEffectPlugin;

impl Plugin for KillEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_effect_handler::<KillEffect>(schedule)
}

fn handler(mut commands: Commands, effect: Res<Handled<KillEffect>>) {
    commands.entity(effect.0.entity).despawn_recursive();
}