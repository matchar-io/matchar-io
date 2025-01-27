use crate::{pool::Pool, room::RoomActor, user::UserActor};
use postbox::Actor;
use refinement::ChannelId;

pub type ChannelPostbox = crate::postbox::Postbox<ChannelActor>;

pub type ChannelCommand = crate::command::Command<ChannelActor>;

pub type ChannelEvent = crate::event::Event<ChannelActor>;

/// 채널
pub struct ChannelActor {
    /// 채널 ID
    pub(crate) channel_id: ChannelId,
    /// 방 목록
    pub(crate) rooms: Pool<RoomActor>,
    /// 모든 유저 목록
    pub(crate) all_users: Pool<UserActor>,
    /// 로비에 있는 유저 목록
    pub(crate) lobby_users: Pool<UserActor>,
}

impl ChannelActor {
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
impl Actor for ChannelActor {
    type Id = ChannelId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.channel_id
    }
}
