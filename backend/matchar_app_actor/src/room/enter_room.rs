use super::{RoomActor, RoomPostbox};
use postbox::{Context, Handler, Message, PostboxResult};
use refinement::{RoomId, UserId};

struct EnterRoomMessage {
    room_id: RoomId,
    user_id: UserId,
}

impl RoomPostbox {
    pub async fn enter_room(&self, room_id: RoomId, user_id: UserId) -> PostboxResult<()> {
        self.postbox
            .ask(EnterRoomMessage { room_id, user_id })
            .await
    }
}

impl Message for EnterRoomMessage {
    type Response = ();
}

#[postbox::async_trait]
impl Handler<EnterRoomMessage> for RoomActor {
    type Response = ();

    async fn handle(
        &mut self,
        message: EnterRoomMessage,
        _context: &mut Context<Self>,
    ) -> Self::Response {
        std::todo!();
    }
}
