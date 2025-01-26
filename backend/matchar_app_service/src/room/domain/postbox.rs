use crate::user::UserPostbox;
use postbox::{Actor, Postbox};
use refinement::{Registry, RoomId, Storable, UserId};

pub struct RoomPostbox {
    pub(crate) postbox: Postbox<Room>,
}

pub struct Room {
    pub(crate) room_id: RoomId,
    pub(crate) name: String,
    pub(crate) host_id: UserId,
    pub(crate) players: Registry<UserPostbox>,
}

pub enum RoomError {
    NotFoundHost,
}

impl From<Postbox<Room>> for RoomPostbox {
    #[inline]
    fn from(postbox: Postbox<Room>) -> Self {
        Self { postbox }
    }
}

impl Storable for RoomPostbox {
    type Id = RoomId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.room_id()
    }
}

impl RoomPostbox {
    #[inline]
    pub const fn room_id(&self) -> RoomId {
        RoomId::new_unchecked(self.postbox.id())
    }
}

impl Room {
    #[inline]
    pub const fn host(&self) -> Option<UserPostbox> {
        self.players.get(&self.host_id).cloned()
    }

    pub fn set_host(&mut self, host_id: UserId) -> bool {
        let can_set = self.players.contains_key(&host_id);
        if can_set {
            self.host_id = host_id;
        }

        can_set
    }
}

impl Actor for Room {
    //
}
