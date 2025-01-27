use crate::{
    common::postbox::Pool,
    game::domain::config::GameConfig,
    user::domain::{UserActor, UserPostbox},
};
use postbox::Actor;
use refinement::{RoomId, UserId};

pub type RoomPostbox = crate::common::postbox::Postbox<RoomActor>;

pub type RoomCommand = crate::common::postbox::Command<RoomActor>;

pub type RoomEvent = crate::common::postbox::Event<RoomActor>;

/// 방
pub struct RoomActor {
    /// 방 ID
    pub(crate) room_id: RoomId,
    /// 방 이름
    pub(crate) name: String,
    /// 비밀번호
    pub(crate) password: Option<String>,
    /// 중도 입장 허용 여부
    pub(crate) late_entry: bool,
    /// 방장 ID
    pub(crate) host_id: UserId,
    /// 플레이어 목록
    pub(crate) players: Pool<UserActor>,
    /// 게임 설정
    pub(crate) config: GameConfig,
}

impl RoomActor {
    pub fn new(
        room_id: RoomId,
        name: String,
        password: Option<String>,
        late_entry: bool,
        host: UserPostbox,
        max_len: usize,
        config: GameConfig,
    ) -> Option<Self> {
        Some(Self {
            room_id,
            name,
            password,
            late_entry,
            host_id: host.id(),
            players: Pool::from_iter(max_len, [host])?,
            config,
        })
    }

    #[inline]
    pub fn locked(&self) -> bool {
        self.password.is_some()
    }

    #[inline]
    pub fn host(&self) -> Option<&UserPostbox> {
        self.players.get(self.host_id)
    }
}

impl Actor for RoomActor {
    type Id = RoomId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.room_id
    }
}
