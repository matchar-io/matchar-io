use super::{EnterCommand, EnterCommandError};
use crate::channel::domain::Channel;
use postbox::{Context, Handler, Message};

#[postbox::async_trait]
impl Handler<EnterCommand> for Channel {
    type Response = <EnterCommand as Message>::Response;

    async fn handle(
        &mut self,
        EnterCommand { user_id }: EnterCommand,
        context: &mut Context<Self>,
    ) -> Self::Response {
        let user = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(EnterCommandError::UserNotFound)?;
        self.all_users.insert(user.clone());
        self.lobby_users.insert(user);

        Ok(())
    }
}
