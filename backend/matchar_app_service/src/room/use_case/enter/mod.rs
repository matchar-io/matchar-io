use crate::room::{Room, RoomPostbox};
use postbox::{Context, Handler, Message, PostboxError};
use refinement::UserId;

pub struct Enter {
    user_id: UserId,
}

pub enum EnterError {
    Postbox(PostboxError),
    UserNotFound,
}

impl RoomPostbox {
    pub async fn enter(&self, user_id: UserId) -> <Enter as Message>::Response {
        match self.postbox.ask(Enter { user_id }).await {
            Ok(response) => response,
            Err(error) => Err(EnterError::Postbox(error)),
        }
    }
}

impl Message for Enter {
    type Response = Result<(), EnterError>;
}

#[postbox::async_trait]
impl Handler<Enter> for Room {
    type Response = <Enter as Message>::Response;

    async fn handle(&mut self, message: Enter, context: &mut Context<Self>) -> Self::Response {
        let user = context
            .registry
            .get(message.user_id.as_uuid())
            .ok_or(EnterError::UserNotFound)?;
        self.players.insert(user);

        Ok(())
    }
}
