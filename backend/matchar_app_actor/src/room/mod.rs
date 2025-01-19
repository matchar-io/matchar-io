mod enter_room;
mod leave_room;

use crate::UserActor;
use postbox::{Actor, Postbox};
use refinement::UserId;
use std::collections::HashMap;

pub struct RoomPostbox {
    postbox: Postbox<RoomActor>,
}

pub struct RoomActor {
    users: HashMap<UserId, UserActor>,
}

impl RoomPostbox {
    #[inline]
    pub fn id(&self) -> uuid::Uuid {
        self.postbox.id()
    }
}

impl Actor for RoomActor {
    //
}
