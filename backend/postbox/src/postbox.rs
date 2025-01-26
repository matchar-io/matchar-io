use crate::{Actor, Context, Handler, Message, Registry};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::{CancellationToken, WaitForCancellationFuture};
use uuid::Uuid;

pub type PostboxResult<T> = Result<T, PostboxError>;

#[async_trait]
trait Delivery<A: Actor>: Send + 'static {
    async fn deliver(&mut self, actor: &mut A, context: &mut Context<A>);
}

pub struct Postbox<A: Actor> {
    id: Uuid,
    poster: mpsc::Sender<Pack<A>>,
    cancellation: CancellationToken,
    termination: CancellationToken,
}

struct InnerPost<M: Message> {
    tx: Option<oneshot::Sender<M::Executed>>,
    message: M,
}

pub(crate) struct PostboxWorker<A: Actor> {
    actor: A,
    postbox: Postbox<A>,
    receiver: mpsc::Receiver<Pack<A>>,
}

struct Pack<A: Actor>(Box<dyn Delivery<A>>);

struct Post<M: Message>(Option<InnerPost<M>>);

#[derive(Debug, Clone, Copy)]
pub enum PostboxError {
    Send,
    Recv,
}

impl<A: Actor> Postbox<A> {
    pub(crate) fn create(id: Uuid, actor: A) -> (Self, PostboxWorker<A>) {
        let (poster, receiver) = mpsc::channel::<Pack<A>>(A::BUFFER_SIZE);
        let postbox = Self {
            id,
            poster,
            cancellation: CancellationToken::new(),
            termination: CancellationToken::new(),
        };
        let worker = PostboxWorker {
            actor,
            postbox: postbox.clone(),
            receiver,
        };

        (postbox, worker)
    }

    #[inline]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    pub async fn ask<M>(&self, message: M) -> PostboxResult<M::Executed>
    where
        A: Handler<M, Executed = M::Executed>,
        M: Message,
    {
        let (tx, rx) = oneshot::channel();
        let pack = Pack(Box::new(Post(Some(InnerPost {
            tx: Some(tx),
            message,
        }))));
        self.poster
            .send(pack)
            .await
            .map_err(|_| PostboxError::Send)?;
        let response = rx.await.map_err(|_| PostboxError::Recv)?;

        Ok(response)
    }

    pub fn tell<M>(&self, message: M) -> PostboxResult<()>
    where
        A: Handler<M, Executed = M::Executed>,
        M: Message,
    {
        let pack = Pack(Box::new(Post(Some(InnerPost { tx: None, message }))));
        self.poster.try_send(pack).map_err(|_| PostboxError::Send)?;

        Ok(())
    }

    #[inline]
    pub fn stop(&self) {
        self.cancellation.cancel();
    }

    #[inline]
    pub fn stopped(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    #[inline]
    pub fn wait_stop(&self) -> WaitForCancellationFuture {
        self.cancellation.cancelled()
    }

    #[inline]
    pub(crate) fn terminate(&self) {
        self.termination.cancel();
    }

    #[inline]
    pub fn terminated(&self) -> bool {
        self.termination.is_cancelled()
    }

    #[inline]
    pub fn wait_terminate(&self) -> WaitForCancellationFuture {
        self.termination.cancelled()
    }
}

impl<A: Actor> Clone for Postbox<A> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            poster: self.poster.clone(),
            cancellation: self.cancellation.clone(),
            termination: self.termination.clone(),
        }
    }
}

impl<A: Actor> PostboxWorker<A> {
    pub fn run(mut self, registry: Arc<Registry>) {
        let mut context = Context {
            postbox: self.postbox.clone(),
            registry,
        };

        tokio::spawn(async move {
            self.actor.started(&mut context).await;

            loop {
                let mut packs = Vec::with_capacity(A::BATCH_SIZE);

                select! {
                    _ = context.wait_stop() => break,
                    _ = self.receiver.recv_many(&mut packs, A::BATCH_SIZE) => {
                        for Pack(mut post) in packs {
                            post.deliver(&mut self.actor, &mut context).await;
                        }
                    }
                }
            }

            self.actor.stopped(&mut context).await;
            context.terminate();
            self.actor.terminated().await;
        });
    }
}

#[async_trait]
impl<A: Actor, M: Message> Delivery<A> for Post<M>
where
    A: Handler<M, Executed = M::Executed>,
{
    async fn deliver(&mut self, actor: &mut A, context: &mut Context<A>) {
        if let Some(InnerPost { tx, message }) = self.0.take() {
            let executed = actor.on_execute(message.clone(), context).await;
            if let Some(tx) = tx {
                if let Err(executed) = tx.send(executed.clone()) {
                    return actor.on_failed(executed).await;
                }
            }

            actor.on_executed(message, executed, context).await;
        }
    }
}
