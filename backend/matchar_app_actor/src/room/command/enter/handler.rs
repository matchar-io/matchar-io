use super::{EnterMessage, EnterMessageError};
use crate::room::RoomActor;
use postbox::{Context, Handler, Message};

#[postbox::async_trait]
impl Handler<EnterMessage> for RoomActor {
    type Executed = <EnterMessage as Message>::Executed;

    async fn on_execute(
        &mut self,
        EnterMessage { user_id, .. }: EnterMessage,
        context: &mut Context<Self>,
    ) -> Self::Executed {
        let user = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(EnterMessageError::UserNotFound)?;
        self.players.insert(user);

        Ok(())
    }
}
