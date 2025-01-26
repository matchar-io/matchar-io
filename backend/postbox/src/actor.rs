use crate::{Postbox, PostboxResult, Registry};
use std::sync::Arc;
use tokio_util::sync::WaitForCancellationFuture;
use uuid::Uuid;

#[async_trait]
#[allow(unused_variables)]
pub trait Actor: Sized + Sync + Send + 'static {
    type Id: Copy + Eq + std::hash::Hash + From<Uuid>;

    const BUFFER_SIZE: usize = 256;
    const BATCH_SIZE: usize = 16;

    fn id(&self) -> Self::Id;

    async fn started(&mut self, context: &mut Context<Self>) {
        //
    }

    async fn stopped(&mut self, context: &mut Context<Self>) {
        //
    }

    async fn terminated(&mut self) {
        //
    }
}

pub trait Message: Clone + Sized + Sync + Send + 'static {
    type Executed: Clone + Send + 'static;
}

#[async_trait]
pub trait Handler<M: Message>: Actor {
    type Executed: Clone + Send + 'static;

    async fn on_execute(&mut self, message: M, context: &mut Context<Self>) -> Self::Executed;

    #[allow(unused_variables)]
    async fn on_executed(
        &mut self,
        message: M,
        executed: Self::Executed,
        context: &mut Context<Self>,
    ) {
        //
    }

    #[allow(unused_variables)]
    async fn on_failed(&mut self, response: M::Executed) {
        //
    }
}

pub struct Context<A: Actor> {
    pub postbox: Postbox<A>,
    pub registry: Arc<Registry>,
}

impl<A: Actor> Context<A> {
    #[inline]
    pub async fn ask<M: Message>(&mut self, message: M) -> PostboxResult<M::Executed>
    where
        A: Handler<M, Executed = M::Executed>,
    {
        self.postbox.ask(message).await
    }

    pub async fn tell<M: Message>(&mut self, message: M) -> PostboxResult<()>
    where
        A: Handler<M, Executed = M::Executed>,
    {
        self.postbox.tell(message)?;

        Ok(())
    }

    #[inline]
    pub fn stop(&self) {
        self.postbox.stop();
    }

    #[inline]
    pub fn stopped(&self) -> bool {
        self.postbox.stopped()
    }

    #[inline]
    pub fn wait_stop(&self) -> WaitForCancellationFuture {
        self.postbox.wait_stop()
    }

    #[inline]
    pub(crate) fn terminate(&self) {
        self.postbox.terminate();
    }

    #[inline]
    pub fn terminated(&self) -> bool {
        self.postbox.terminated()
    }

    #[inline]
    pub fn wait_terminate(&self) -> WaitForCancellationFuture {
        self.postbox.wait_terminate()
    }
}
