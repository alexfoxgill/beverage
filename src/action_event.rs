use bevy::prelude::*;

use hex2d::{Direction as HexDirection, *};

use crate::effects::{
    energy_cost_effect::{ActionCost, EnergyCostEffect},
    face_effect::FaceEffect,
    kill_effect::KillEffect,
    move_effect::MoveEffect,
    *,
};

use super::common::*;

#[derive(Debug)]
pub struct ActionEvent(Box<dyn Action>);

impl Action for ActionEvent {
    fn entity(&self) -> Entity {
        self.0.entity()
    }

    fn effects(&self) -> Vec<EffectEvent> {
        self.0.effects()
    }
}

pub trait Action: Send + Sync + std::fmt::Debug {
    fn entity(&self) -> Entity;
    fn effects(&self) -> Vec<EffectEvent>;
}

pub fn process_action_events(
    mut actors: Query<&Actor>,
    mut events: EventReader<ActionEvent>,
    mut effects: EventWriter<EffectEvent>,
) {
    for action in events.iter() {
        if let Ok(actor) = actors.get_mut(action.entity()) {
            for effect in action.effects().iter() {
                if let Some(energy_cost) = effect.as_effect::<EnergyCostEffect>() {
                    match energy_cost.cost {
                        ActionCost::Fixed(x) if x > actor.actions_remaining => continue,
                        _ => (),
                    }
                }
            }

            for effect in action.effects().into_iter() {
                effects.send(effect);
            }
        }
    }
}

pub enum Effect {
    MoveSelf(Coordinate),
    RotateSelf(HexDirection),
    Kill(Entity),
}

#[derive(Debug)]
pub struct MoveAction {
    entity: Entity,
    to: Coordinate,
}

impl MoveAction {
    pub fn event(entity: Entity, to: Coordinate) -> ActionEvent {
        ActionEvent(Box::new(MoveAction { entity, to }))
    }
}
impl Action for MoveAction {
    fn entity(&self) -> Entity {
        self.entity
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![
            EnergyCostEffect::event(self.entity, ActionCost::Fixed(1)),
            MoveEffect::event(self.entity, self.to),
        ]
    }
}

#[derive(Debug)]
pub struct RotateAction {
    entity: Entity,
    to: HexDirection,
}

impl RotateAction {
    pub fn event(entity: Entity, to: HexDirection) -> ActionEvent {
        ActionEvent(Box::new(RotateAction { entity, to }))
    }
}

impl Action for RotateAction {
    fn entity(&self) -> Entity {
        self.entity
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![
            EnergyCostEffect::event(self.entity, ActionCost::None),
            FaceEffect::event(self.entity, self.to),
        ]
    }
}

#[derive(Debug)]
pub struct EndTurnAction {
    entity: Entity,
}

impl EndTurnAction {
    pub fn event(entity: Entity) -> ActionEvent {
        ActionEvent(Box::new(EndTurnAction { entity }))
    }
}

impl Action for EndTurnAction {
    fn entity(&self) -> Entity {
        self.entity
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![EnergyCostEffect::event(self.entity, ActionCost::All)]
    }
}

#[derive(Debug)]
pub struct AttackAction {
    attacker: Entity,
    victim: Entity,
}

impl AttackAction {
    pub fn event(attacker: Entity, victim: Entity) -> ActionEvent {
        ActionEvent(Box::new(AttackAction { attacker, victim }))
    }
}

impl Action for AttackAction {
    fn entity(&self) -> Entity {
        self.attacker
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![
            EnergyCostEffect::event(self.attacker, ActionCost::All),
            KillEffect::event(self.victim),
        ]
    }
}
