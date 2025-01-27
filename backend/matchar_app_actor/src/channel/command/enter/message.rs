use crate::channel::ChannelCommand;
use postbox::{Message, PostOffice, PostboxError};
use refinement::{ChannelId, UserId};

#[derive(Clone, Deserialize)]
pub struct EnterMessage {
    pub(crate) channel_id: ChannelId,
    pub(crate) user_id: UserId,
}

#[derive(Debug, Clone, Error)]
pub enum EnterMessageError {
    #[error("Postbox error")]
    Postbox(PostboxError),
    #[error("Channel not found")]
    ChannelNotFound,
    #[error("User not found")]
    UserNotFound,
}

impl EnterMessage {
    pub async fn tell(self, office: PostOffice) -> Result<(), EnterMessageError> {
        let Some(channel) = office.find(self.channel_id).map(ChannelCommand::new) else {
            return Err(EnterMessageError::ChannelNotFound);
        };
        channel
            .ask(self)
            .await
            .unwrap_or_else(|error| Err(EnterMessageError::Postbox(error)))?;

        Ok(())
    }
}

impl Message for EnterMessage {
    type Executed = Result<(), EnterMessageError>;
}
