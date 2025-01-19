use crate::{Actor, Postbox, Registry};
use std::sync::Arc;

pub struct PostOffice {
    registry: Arc<Registry>,
}

impl PostOffice {
    pub fn new() -> Self {
        let registry = Arc::new(Registry::new());

        Self { registry }
    }

    pub async fn spawn<A: Actor>(&mut self, actor: A) -> Postbox<A> {
        let (postbox, worker) = Postbox::create(actor);
        if let Some(registry) = Arc::get_mut(&mut self.registry) {
            registry.insert(postbox.clone());
        }
        worker.run(self.registry.clone());

        postbox
    }

    #[inline]
    pub fn find<A: Actor>(&self, id: uuid::Uuid) -> Option<Postbox<A>> {
        self.registry.get(id)
    }
}
