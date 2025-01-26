use crate::user::domain::{User, UserEvent};
use postbox::{Context, Handler, Message, PostboxError, PostboxResult};

pub trait Event: serde::Serialize + Clone + Sync + Send + 'static {
    const TYPE: &'static str;
}

#[derive(Clone)]
pub struct EmitEvent<E> {
    event: E,
}

#[derive(Clone)]
pub enum EmitEventError {
    Postbox(PostboxError),
    Emitter(tunnel::EmitterError),
}

impl UserEvent {
    pub fn emit_event<E: Event>(&self, event: E) -> PostboxResult<()> {
        self.tell(EmitEvent { event })
    }
}

impl<E> Message for EmitEvent<E>
where
    E: Event,
{
    type Executed = Result<(), EmitEventError>;
}

#[postbox::async_trait]
impl<E> Handler<EmitEvent<E>> for User
where
    E: Event,
{
    type Executed = <EmitEvent<E> as Message>::Executed;

    async fn on_execute(
        &mut self,
        EmitEvent { event }: EmitEvent<E>,
        _context: &mut Context<Self>,
    ) -> Self::Executed {
        self.emitter
            .emit(E::TYPE, event)
            .await
            .map_err(EmitEventError::Emitter)?;

        Ok(())
    }
}
