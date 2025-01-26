use crate::channel::domain::ChannelEvent;
use postbox::{Message, PostboxResult};
use refinement::RoomId;

pub struct RoomCountUpdatedEvent {
    pub(crate) room: Room,
}

pub struct Room {
    pub(crate) room_id: RoomId,
    pub(crate) count: usize,
}

impl ChannelEvent {
    pub fn room_count_updated(
        &self,
        room_id: RoomId,
        count: usize,
    ) -> <RoomCountUpdatedEvent as Message>::Response {
        self.tell(RoomCountUpdatedEvent {
            room: Room { room_id, count },
        })
    }
}

impl Message for RoomCountUpdatedEvent {
    type Response = PostboxResult<()>;
}
