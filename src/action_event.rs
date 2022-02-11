use std::time::Duration;

use bevy::prelude::*;

use bevy_easings::{Ease, EaseFunction, EasingType};
use hex2d::{Direction as HexDirection, *};

use super::common::*;

pub enum ActionCost {
    All,
    Fixed(u8),
    None,
}

pub struct ActionEvent(Box<dyn Action>);

impl Action for ActionEvent {
    fn entity(&self) -> Entity {
        self.0.entity()
    }

    fn cost(&self) -> ActionCost {
        self.0.cost()
    }

    fn effects(&self) -> Vec<Effect> {
        self.0.effects()
    }
}

pub trait Action: Send + Sync {
    fn entity(&self) -> Entity;
    fn cost(&self) -> ActionCost;
    fn effects(&self) -> Vec<Effect>;
}

pub fn process_action_events(
    mut commands: Commands,
    mut actors: Query<(&mut Actor, &mut HexPos, &mut Facing)>,
    mut events: EventReader<ActionEvent>,
) {
    for action in events.iter() {
        if let Ok((mut actor, mut pos, mut facing)) = actors.get_mut(action.entity()) {
            match action.cost() {
                ActionCost::All => actor.actions_remaining = 0,
                ActionCost::Fixed(x) => actor.actions_remaining -= x,
                ActionCost::None => (),
            }

            for effect in action.effects().iter() {
                match effect {
                    Effect::Move(_, to) => {
                        pos.0 = *to;
                    }
                    Effect::Rotate(_, to) => {
                        facing.0 = *to;
                    }
                    Effect::Die(e) => {
                        commands.entity(*e).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub enum Effect {
    Move(Entity, Coordinate),
    Rotate(Entity, HexDirection),
    Die(Entity),
}

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

    fn effects(&self) -> Vec<Effect> {
        vec![Effect::Move(self.entity, self.to)]
    }
}

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

    fn effects(&self) -> Vec<Effect> {
        vec![Effect::Rotate(self.entity, self.to)]
    }
}

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

    fn effects(&self) -> Vec<Effect> {
        vec![]
    }
}

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

    fn effects(&self) -> Vec<Effect> {
        vec![Effect::Die(self.victim)]
    }
}
