use postbox::{Actor, Postbox};

pub struct WordChainGamePostbox {
    postbox: Postbox<WordChainGameActor>,
}

pub struct WordChainGameActor {
    //
}

impl WordChainGamePostbox {
    #[inline]
    pub fn id(&self) -> uuid::Uuid {
        self.postbox.id()
    }
}

impl Actor for WordChainGameActor {
    //
}
