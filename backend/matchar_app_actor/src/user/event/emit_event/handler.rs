use super::{EmitEvent, EmitEventError, Event};
use crate::user::UserActor;
use postbox::{Context, Handler, Message};

#[postbox::async_trait]
impl<E> Handler<EmitEvent<E>> for UserActor
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
            .event(E::TYPE, event)
            .await
            .map_err(EmitEventError::Emitter)?;

        Ok(())
    }
}
