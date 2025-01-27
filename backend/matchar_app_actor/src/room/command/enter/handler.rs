use super::{EnterCommand, EnterError};
use crate::room::RoomActor;
use postbox::{Context, Handler, Message};

#[postbox::async_trait]
impl Handler<EnterCommand> for RoomActor {
    type Executed = <EnterCommand as Message>::Executed;

    async fn on_execute(
        &mut self,
        EnterCommand { user_id }: EnterCommand,
        context: &mut Context<Self>,
    ) -> Self::Executed {
        let user = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(EnterError::UserNotFound)?;
        self.players.insert(user);

        Ok(())
    }
}
