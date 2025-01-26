use crate::room::{Room, RoomPostbox};
use postbox::{Context, Handler, Message, PostboxError};
use refinement::UserId;

pub struct EnterRoom {
    user_id: UserId,
}

pub enum EnterRoomError {
    Postbox(PostboxError),
    UserNotFound,
}

impl RoomPostbox {
    pub async fn enter(&self, user_id: UserId) -> <EnterRoom as Message>::Response {
        match self.postbox.ask(EnterRoom { user_id }).await {
            Ok(response) => response,
            Err(error) => Err(EnterRoomError::Postbox(error)),
        }
    }
}

impl Message for EnterRoom {
    type Response = Result<(), EnterRoomError>;
}

#[postbox::async_trait]
impl Handler<EnterRoom> for Room {
    type Response = <EnterRoom as Message>::Response;

    async fn handle(&mut self, message: EnterRoom, context: &mut Context<Self>) -> Self::Response {
        let user = context
            .registry
            .get(message.user_id.as_uuid())
            .ok_or(EnterRoomError::UserNotFound)?;
        self.players.insert(user);

        Ok(())
    }
}
