use std::hash::Hash;
use std::marker::PhantomData;

use bevy::prelude::*;

use std::collections::hash_map::*;
use std::collections::hash_set::*;

pub struct ComponentIndex<C: Component + Clone + Eq + Hash> {
    component_to_entities: HashMap<C, HashSet<Entity>>,
    entity_to_component: HashMap<Entity, C>,
}

impl<C: Component + Clone + Eq + Hash> Default for ComponentIndex<C> {
    fn default() -> Self {
        Self {
            component_to_entities: Default::default(),
            entity_to_component: Default::default(),
        }
    }
}

impl<C: Component + Clone + Eq + Hash> ComponentIndex<C> {
    pub fn update(&mut self, e: Entity, new: C) {
        self.remove(e);

        let entity_set = match self.component_to_entities.entry(new.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(HashSet::new()),
        };
        entity_set.insert(e);
        self.entity_to_component.insert(e, new);
    }

    pub fn remove(&mut self, e: Entity) {
        if let Some(old) = self.entity_to_component.remove(&e) {
            if let Some(entities) = self.component_to_entities.get_mut(&old) {
                entities.remove(&e);
                if entities.is_empty() {
                    self.component_to_entities.remove(&old);
                }
            }
        }
    }

    pub fn get_entities(&self, component: &C) -> Option<&HashSet<Entity>> {
        self.component_to_entities.get(component)
    }

    pub fn plugin() -> ComponentIndexPlugin<C> {
        ComponentIndexPlugin {
            phantom: PhantomData,
        }
    }
}

pub fn index_changed<C: Component + Clone + Eq + Hash>(
    mut index: ResMut<ComponentIndex<C>>,
    changed: Query<(Entity, &C), Changed<C>>,
    removed: RemovedComponents<C>,
) {
    changed.for_each(|(e, c)| {
        index.update(e, c.clone());
    });

    for e in removed.iter() {
        index.remove(e);
    }
}

pub struct ComponentIndexPlugin<C: Component + Clone + Eq + Hash> {
    phantom: PhantomData<C>,
}

impl<C: Component + Clone + Eq + Hash> Plugin for ComponentIndexPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ComponentIndex<C>>()
            .add_system_to_stage(CoreStage::PostUpdate, index_changed::<C>);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, Clone, PartialEq, Eq, Hash)]
    struct TestComponent(u8);

    #[test]
    fn component_index_stores_entity() {
        let mut index = ComponentIndex::<TestComponent>::default();

        let entity = Entity::from_raw(1);
        let c = TestComponent(0);
        index.update(entity, c.clone());

        let res = index.get_entities(&c).unwrap();
        assert_eq!(res.len(), 1);
        assert!(res.contains(&entity));
    }

    #[test]
    fn component_index_updates_entity() {
        let mut index = ComponentIndex::<TestComponent>::default();

        let entity = Entity::from_raw(1);
        let c1 = TestComponent(0);
        let c2 = TestComponent(0);
        index.update(entity, c1.clone());
        index.update(entity, c2.clone());

        let res = index.get_entities(&c2).unwrap();
        assert_eq!(res.len(), 1);
        assert!(res.contains(&entity));
    }

    #[test]
    fn component_index_removes_entity() {
        let mut index = ComponentIndex::<TestComponent>::default();

        let entity = Entity::from_raw(1);
        let c = TestComponent(0);
        index.update(entity, c.clone());
        index.remove(entity);

        let res = index.get_entities(&c);
        assert!(res.is_none());
    }

    #[test]
    fn component_index_updates_old_component() {
        let mut index = ComponentIndex::<TestComponent>::default();

        let entity = Entity::from_raw(1);
        let c1 = TestComponent(0);
        let c2 = TestComponent(1);
        index.update(entity, c1.clone());
        index.update(entity, c2.clone());

        let res = index.get_entities(&c1);
        assert_eq!(res, None);
    }

    #[test]
    fn component_index_allows_multiple_entities() {
        let mut index = ComponentIndex::<TestComponent>::default();

        let e1 = Entity::from_raw(1);
        let e2 = Entity::from_raw(2);
        let c = TestComponent(0);
        index.update(e1, c.clone());
        index.update(e2, c.clone());

        let res = index.get_entities(&c).unwrap();
        assert!(res.contains(&e1));
        assert!(res.contains(&e2));
    }
}
