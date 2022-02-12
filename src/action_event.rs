use bevy::prelude::*;

use hex2d::{Direction as HexDirection, *};

use crate::effects::{
    face_effect::FaceEffect, kill_effect::KillEffect, move_effect::MoveEffect, *,
};

use super::common::*;

pub enum ActionCost {
    All,
    Fixed(u8),
    None,
}

#[derive(Debug)]
pub struct ActionEvent(Box<dyn Action>);

impl Action for ActionEvent {
    fn entity(&self) -> Entity {
        self.0.entity()
    }

    fn cost(&self) -> ActionCost {
        self.0.cost()
    }

    fn effects(&self) -> Vec<EffectEvent> {
        self.0.effects()
    }
}

pub trait Action: Send + Sync + std::fmt::Debug {
    fn entity(&self) -> Entity;
    fn cost(&self) -> ActionCost;
    fn effects(&self) -> Vec<EffectEvent>;
}

pub fn process_action_events(
    mut actors: Query<(&mut Actor, &HexPos, &Facing)>,
    mut events: EventReader<ActionEvent>,
    mut effects: EventWriter<EffectEvent>,
) {
    for action in events.iter() {
        if let Ok((mut actor, _pos, _facing)) = actors.get_mut(action.entity()) {
            match action.cost() {
                ActionCost::All => actor.actions_remaining = 0,
                ActionCost::Fixed(x) if x <= actor.actions_remaining => {
                    actor.actions_remaining -= x
                }
                ActionCost::Fixed(_) => continue,
                ActionCost::None => (),
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

    fn cost(&self) -> ActionCost {
        ActionCost::Fixed(1)
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![MoveEffect::event(self.entity, self.to)]
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

    fn cost(&self) -> ActionCost {
        ActionCost::None
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![FaceEffect::event(self.entity, self.to)]
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

    fn cost(&self) -> ActionCost {
        ActionCost::All
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![]
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

    fn cost(&self) -> ActionCost {
        ActionCost::All
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![KillEffect::event(self.victim)]
    }
}
