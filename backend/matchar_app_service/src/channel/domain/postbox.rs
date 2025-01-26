use crate::{room::RoomPostbox, user::UserPostbox};
use postbox::{Actor, Postbox};
use refinement::{ChannelId, Pool};

#[derive(Clone)]
pub struct ChannelPostbox {
    pub(crate) postbox: Postbox<Channel>,
}

/// 채널
pub(crate) struct Channel {
    /// 채널 ID
    pub(crate) channel_id: ChannelId,
    /// 유저 목록
    pub(crate) users: Pool<UserPostbox>,
    /// 방 목록
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
    pub fn new(channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            users: Pool::new(1000),
            rooms: Pool::new(125),
        }
    }
}
