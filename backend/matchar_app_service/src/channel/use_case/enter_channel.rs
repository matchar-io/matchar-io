use crate::channel::{Channel, ChannelPostbox};
use postbox::{Context, Handler, Message, PostboxError};
use refinement::UserId;

pub struct EnterChannel {
    user_id: UserId,
}

pub enum EnterChannelError {
    Postbox(PostboxError),
    UserNotFound,
}

impl ChannelPostbox {
    pub async fn enter(&self, user_id: UserId) -> <EnterChannel as Message>::Response {
        match self.postbox.ask(EnterChannel { user_id }).await {
            Ok(response) => response,
            Err(error) => Err(EnterChannelError::Postbox(error)),
        }
    }
}

impl Message for EnterChannel {
    type Response = Result<(), EnterChannelError>;
}

#[postbox::async_trait]
impl Handler<EnterChannel> for Channel {
    type Response = <EnterChannel as Message>::Response;

    async fn handle(
        &mut self,
        EnterChannel { user_id }: EnterChannel,
        context: &mut Context<Self>,
    ) -> Self::Response {
        let user = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(EnterChannelError::UserNotFound)?;
        self.users.insert(user);

        Ok(())
    }
}
