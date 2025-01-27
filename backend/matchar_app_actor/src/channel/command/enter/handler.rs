use super::{EnterMessage, EnterMessageError};
use crate::channel::ChannelActor;
use postbox::{Context, Handler, Message};

#[postbox::async_trait]
impl Handler<EnterMessage> for ChannelActor {
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
        self.all_users.insert(user.clone());
        self.lobby_users.insert(user);

        Ok(())
    }
}
