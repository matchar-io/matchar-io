use crate::{Actor, Postbox};
use std::collections::HashMap;
use uuid::Uuid;

type DynamicPostbox = Box<dyn std::any::Any + Send + Sync>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorId {
    type_id: std::any::TypeId,
    id: Uuid,
}

pub struct Registry {
    map: HashMap<ActorId, DynamicPostbox>,
}

impl ActorId {
    pub fn new<A: Actor>(id: Uuid) -> Self {
        Self {
            type_id: std::any::TypeId::of::<A>(),
            id,
        }
    }
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
            .insert(ActorId::new::<A>(postbox.id()), Box::new(postbox));
    }

    pub fn remove<A: Actor>(&mut self, id: Uuid) {
        self.map.remove(&ActorId::new::<A>(id));
    }

    pub fn get<A: Actor>(&self, id: Uuid) -> Option<Postbox<A>> {
        self.map
            .get(&ActorId::new::<A>(id))
            .and_then(|postbox| postbox.downcast_ref().cloned())
    }

    pub fn contains<A: Actor>(&self, id: Uuid) -> bool {
        self.map.contains_key(&ActorId::new::<A>(id))
    }

    pub fn contains_actor<A: Actor>(&self) -> bool {
        let type_id = std::any::TypeId::of::<A>();

        self.map
            .iter()
            .any(|(actor_id, _)| actor_id.type_id == type_id)
    }
}
