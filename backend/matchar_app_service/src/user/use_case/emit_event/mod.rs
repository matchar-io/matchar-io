use crate::user::domain::{User, UserEvent};
use postbox::{Context, Handler, Message, PostboxError, PostboxResult};

pub trait Event: serde::Serialize + Sync + Send + 'static {
    const TYPE: &'static str;
}

pub struct EmitEvent<E> {
    event: E,
}

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
    type Response = Result<(), EmitEventError>;
}

#[postbox::async_trait]
impl<E> Handler<EmitEvent<E>> for User
where
    E: Event,
{
    type Response = <EmitEvent<E> as Message>::Response;

    async fn handle(
        &mut self,
        EmitEvent { event }: EmitEvent<E>,
        _context: &mut Context<Self>,
    ) -> Self::Response {
        self.emitter
            .emit(E::TYPE, event)
            .await
            .map_err(EmitEventError::Emitter)?;

        Ok(())
    }
}
