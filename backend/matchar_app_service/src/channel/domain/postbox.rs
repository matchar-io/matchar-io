use crate::{room::RoomPostbox, user::UserPostbox};
use postbox::{Actor, Postbox};
use refinement::{ChannelId, Pool};

#[derive(Clone)]
pub struct ChannelPostbox {
    pub(crate) postbox: Postbox<Channel>,
}

pub(crate) struct Channel {
    pub(crate) channel_id: ChannelId,
    pub(crate) name: String,
    pub(crate) users: Pool<UserPostbox>,
    pub(crate) rooms: Pool<RoomPostbox>,
}

impl ChannelPostbox {
    #[inline]
    pub const fn channel_id(&self) -> ChannelId {
        ChannelId::new_unchecked(self.postbox.id())
    }
}

impl From<Postbox<Channel>> for ChannelPostbox {
    #[inline]
    fn from(postbox: Postbox<Channel>) -> Self {
        Self { postbox }
    }
}
#[postbox::async_trait]
impl Actor for Channel {
    //
}

impl Channel {
    pub fn new(channel_id: ChannelId, name: String) -> Self {
        Self {
            channel_id,
            name,
            users: Pool::new(1000),
            rooms: Pool::new(125),
        }
    }
}
