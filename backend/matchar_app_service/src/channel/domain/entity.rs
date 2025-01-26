use crate::{common::postbox::Pool, room::domain::Room, user::domain::User};
use postbox::Actor;
use refinement::ChannelId;

pub type ChannelPostbox = crate::common::postbox::Postbox<Channel>;

pub type ChannelCommand = crate::common::postbox::Command<Channel>;

pub type ChannelEvent = crate::common::postbox::Event<Channel>;

/// 채널
pub struct Channel {
    /// 채널 ID
    pub(crate) channel_id: ChannelId,
    /// 방 목록
    pub(crate) rooms: Pool<Room>,
    /// 모든 유저 목록
    pub(crate) all_users: Pool<User>,
    /// 로비에 있는 유저 목록
    pub(crate) lobby_users: Pool<User>,
}

impl Channel {
    pub fn new(channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            rooms: Pool::new(125),
            all_users: Pool::new(1000),
            lobby_users: Pool::new(1000),
        }
    }
}

#[postbox::async_trait]
impl Actor for Channel {
    type Id = ChannelId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.channel_id
    }
}
