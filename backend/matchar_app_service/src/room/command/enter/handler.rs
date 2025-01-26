use super::{EnterCommand, EnterError};
use crate::room::domain::Room;
use postbox::{Context, Handler, Message};

#[postbox::async_trait]
impl Handler<EnterCommand> for Room {
    type Response = <EnterCommand as Message>::Response;

    async fn handle(
        &mut self,
        EnterCommand { user_id }: EnterCommand,
        context: &mut Context<Self>,
    ) -> Self::Response {
        let user = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(EnterError::UserNotFound)?;
        self.players.insert(user);

        Ok(())
    }
}
