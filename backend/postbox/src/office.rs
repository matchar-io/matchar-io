use crate::{Actor, Broadcast, Postbox, Registry};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostOffice {
    registry: Arc<Registry>,
}

impl PostOffice {
    pub fn new() -> Self {
        let registry = Arc::new(Registry::new());

        Self { registry }
    }

    pub fn spawn<A: Actor>(&mut self, id: Uuid, actor: A) -> Postbox<A> {
        let (postbox, worker) = Postbox::create(id, actor);
        if let Some(registry) = Arc::get_mut(&mut self.registry) {
            registry.insert(postbox.clone());
        }
        worker.run(self.registry.clone());

        postbox
    }

    #[inline]
    pub fn find<A>(&self, id: impl Into<uuid::Uuid>) -> Option<Postbox<A>>
    where
        A: Actor,
    {
        self.registry.get(id.into())
    }

    pub fn broadcast<A>(&self) -> Broadcast<impl Iterator<Item = Postbox<A>> + '_>
    where
        A: Actor,
    {
        Broadcast::new(self.registry.iter_actor())
    }
}
