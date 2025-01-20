use crate::{Actor, Handler, Message, Postbox, PostboxResult};

pub struct Broadcast<I> {
    iter: I,
}

impl<I> Broadcast<I> {
    pub const fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I, A> Broadcast<I>
where
    I: Iterator<Item = Postbox<A>>,
    A: Actor,
{
    pub fn filter<F>(self, f: F) -> Broadcast<impl Iterator<Item = Postbox<A>>>
    where
        F: Fn(uuid::Uuid) -> bool,
    {
        Broadcast::new(self.iter.filter(move |actor| f(actor.id())))
    }

    pub async fn ask<M>(self, message: M) -> PostboxResult<()>
    where
        A: Handler<M, Response = M::Response>,
        M: Message + Clone,
    {
        for postbox in self.iter {
            let _ = postbox.ask(message.clone()).await?;
        }

        Ok(())
    }
}
