use crate::room::RoomCommand;
use postbox::{Message, PostOffice, PostboxError};
use refinement::{RoomId, UserId};

#[derive(Clone, Deserialize)]
pub struct EnterMessage {
    pub(crate) room_id: RoomId,
    pub(crate) user_id: UserId,
}

#[derive(Debug, Clone, Error)]
pub enum EnterMessageError {
    #[error("Postbox error")]
    Postbox(PostboxError),
    #[error("Room not found")]
    RoomNotFound,
    #[error("User not found")]
    UserNotFound,
}

impl EnterMessage {
    pub async fn tell(self, office: PostOffice) -> Result<(), EnterMessageError> {
        let room = office
            .find(self.room_id)
            .map(RoomCommand::new)
            .ok_or(EnterMessageError::RoomNotFound)?;
        room.ask(self)
            .await
            .unwrap_or_else(|error| Err(EnterMessageError::Postbox(error)))
    }
}

impl Message for EnterMessage {
    type Executed = Result<(), EnterMessageError>;
}
