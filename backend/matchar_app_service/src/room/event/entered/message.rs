use crate::room::domain::RoomEvent;
use postbox::{Message, PostboxError};
use refinement::{RoomId, UserId, UserName};

pub struct EnteredEvent {
    pub(crate) room_id: RoomId,
    pub(crate) user: User,
}

pub struct User {
    pub(crate) user_id: UserId,
    pub(crate) name: UserName,
}

pub enum EnteredEventError {
    Postbox(PostboxError),
}

impl RoomEvent {
    pub async fn enter(&self, room_id: RoomId, user: User) -> <EnteredEvent as Message>::Response {
        match self.postbox.ask(EnteredEvent { room_id, user }).await {
            Ok(response) => response,
            Err(error) => Err(EnteredEventError::Postbox(error)),
        }
    }
}

impl Message for EnteredEvent {
    type Response = Result<(), EnteredEventError>;
}
