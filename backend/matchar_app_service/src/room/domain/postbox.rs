use crate::player::PlayerPostbox;
use postbox::{Actor, Postbox};
use refinement::{PlayerId, RoomId};
use std::collections::HashMap;

pub struct RoomPostbox {
    pub(crate) postbox: Postbox<Room>,
}

pub struct Room {
    pub(crate) room_id: RoomId,
    pub(crate) name: String,
    pub(crate) players: HashMap<PlayerId, PlayerPostbox>,
}

impl From<Postbox<Room>> for RoomPostbox {
    #[inline]
    fn from(postbox: Postbox<Room>) -> Self {
        Self { postbox }
    }
}

impl Actor for Room {
    //
}
