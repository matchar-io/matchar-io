use crate::channel::domain::ChannelEvent;
use postbox::{Message, PostboxResult};
use refinement::ChannelId;

pub struct EnteredEvent {
    pub(crate) channel_id: ChannelId,
    pub(crate) count: usize,
}

impl ChannelEvent {
    pub fn enter(
        &self,
        channel_id: ChannelId,
        count: usize,
    ) -> <EnteredEvent as Message>::Response {
        self.tell(EnteredEvent { channel_id, count })
    }
}

impl Message for EnteredEvent {
    type Response = PostboxResult<()>;
}
