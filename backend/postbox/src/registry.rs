use crate::{Actor, Postbox};
use std::{any::TypeId, collections::HashMap};
use uuid::Uuid;

type DynamicPostbox = Box<dyn std::any::Any + Send + Sync>;

pub struct Registry {
    map: HashMap<TypeId, HashMap<Uuid, DynamicPostbox>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn insert<A: Actor>(&mut self, postbox: Postbox<A>) {
        self.map
            .entry(TypeId::of::<A>())
            .or_insert_with(HashMap::new)
            .insert(postbox.id(), Box::new(postbox));
    }

    pub fn remove<A: Actor>(&mut self, id: Uuid) {
        if let Some(map) = self.map.get_mut(&TypeId::of::<A>()) {
            map.remove(&id);
        }
    }

    pub fn get<A: Actor>(&self, id: Uuid) -> Option<Postbox<A>> {
        self.map
            .get(&TypeId::of::<A>())
            .and_then(|map| map.get(&id))
            .and_then(|postbox| postbox.downcast_ref().cloned())
    }

    pub fn contains<A: Actor>(&self, id: Uuid) -> bool {
        self.map
            .get(&TypeId::of::<A>())
            .map_or(false, |map| map.contains_key(&id))
    }

    pub fn contains_actor<A: Actor>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<A>())
    }

    pub fn iter_actor<A: Actor>(&self) -> impl Iterator<Item = Postbox<A>> + '_ {
        self.map
            .get(&TypeId::of::<A>())
            .into_iter()
            .flat_map(|map| {
                map.values()
                    .flat_map(|postbox| postbox.downcast_ref().cloned())
            })
    }
}
