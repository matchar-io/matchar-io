use crate::room::RoomEvent;
use postbox::{Message, PostboxResult};
use refinement::{RoomId, UserId, UserName};

#[derive(Clone)]
pub struct EnteredEvent {
    pub(crate) user: User,
    pub(crate) room_id: RoomId,
}

#[derive(Clone)]
pub struct User {
    pub(crate) user_id: UserId,
    pub(crate) name: UserName,
}

impl RoomEvent {
    pub fn enter(&self, user: User, room_id: RoomId) -> <EnteredEvent as Message>::Executed {
        self.tell(EnteredEvent { user, room_id })
    }
}

impl Message for EnteredEvent {
    type Executed = PostboxResult<()>;
}
