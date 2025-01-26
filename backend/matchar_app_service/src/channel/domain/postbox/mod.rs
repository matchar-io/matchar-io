use crate::{room::RoomPostbox, user::UserPostbox};
use postbox::{Actor, Postbox};
use refinement::{ChannelId, Registry};

pub struct ChannelPostbox {
    pub(crate) postbox: Postbox<Channel>,
}

pub(crate) struct Channel {
    pub(crate) channel_id: ChannelId,
    pub(crate) name: String,
    pub(crate) users: Registry<UserPostbox>,
    pub(crate) rooms: Registry<RoomPostbox>,
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

impl ChannelPostbox {
    #[inline]
    pub const fn channel_id(&self) -> ChannelId {
        ChannelId::new_unchecked(self.postbox.id())
    }
}
