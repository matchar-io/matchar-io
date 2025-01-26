use crate::{game::domain::config::GameConfig, user::UserPostbox};
use postbox::{Actor, Postbox};
use refinement::{Item, Pool, RoomId, UserId};

#[derive(Clone)]
pub struct RoomPostbox {
    pub(crate) postbox: Postbox<Room>,
}

/// 방
pub struct Room {
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
    pub(crate) players: Pool<UserPostbox>,
    /// 게임 설정
    pub(crate) config: GameConfig,
}

pub enum RoomError {
    NotFoundHost,
}

impl RoomPostbox {
    #[inline]
    pub const fn room_id(&self) -> RoomId {
        RoomId::new_unchecked(self.postbox.id())
    }
}

impl From<Postbox<Room>> for RoomPostbox {
    #[inline]
    fn from(postbox: Postbox<Room>) -> Self {
        Self { postbox }
    }
}

impl Item for RoomPostbox {
    type Id = RoomId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.room_id()
    }
}

impl Room {
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
            host_id: host.user_id(),
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

impl Actor for Room {
    //
}
