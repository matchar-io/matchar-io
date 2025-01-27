use super::{EnterCommand, EnterCommandError};
use crate::channel::domain::ChannelActor;
use postbox::{Context, Handler, Message};

#[postbox::async_trait]
impl Handler<EnterCommand> for ChannelActor {
    type Executed = <EnterCommand as Message>::Executed;

    async fn on_execute(
        &mut self,
        EnterCommand { user_id }: EnterCommand,
        context: &mut Context<Self>,
    ) -> Self::Executed {
        let user = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(EnterCommandError::UserNotFound)?;
        self.all_users.insert(user.clone());
        self.lobby_users.insert(user);

        Ok(())
    }
}
