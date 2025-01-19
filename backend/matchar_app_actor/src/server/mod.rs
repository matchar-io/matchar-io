mod create_room;

use crate::RoomActor;
use postbox::{Actor, Postbox};
use refinement::ServerId;
use std::collections::HashMap;

pub struct ServerPostbox {
    postbox: Postbox<ServerActor>,
}

pub struct ServerActor {
    rooms: HashMap<ServerId, RoomActor>,
}

impl ServerPostbox {
    #[inline]
    pub fn id(&self) -> uuid::Uuid {
        self.postbox.id()
    }
}

impl Actor for ServerActor {
    //
}
