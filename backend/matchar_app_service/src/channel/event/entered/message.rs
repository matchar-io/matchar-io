use crate::channel::domain::ChannelEvent;
use postbox::{Message, PostboxError};
use refinement::ChannelId;

pub struct EnteredEvent {
    pub(crate) channel_id: ChannelId,
    pub(crate) count: usize,
}

pub enum EnteredEventError {
    Postbox(PostboxError),
}

impl ChannelEvent {
    pub async fn enter(
        &self,
        channel_id: ChannelId,
        count: usize,
    ) -> <EnteredEvent as Message>::Response {
        match self.postbox.ask(EnteredEvent { channel_id, count }).await {
            Ok(response) => response,
            Err(error) => Err(EnteredEventError::Postbox(error)),
        }
    }
}

impl Message for EnteredEvent {
    type Response = Result<(), EnteredEventError>;
}
