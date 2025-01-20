use super::{RoomActor, RoomPostbox};
use postbox::{Context, Handler, Message, PostboxResult};
use refinement::UserId;

struct LeaveRoomMessage {
    user_id: UserId,
}

impl RoomPostbox {
    pub async fn leave_room(&self, user_id: UserId) -> PostboxResult<()> {
        self.postbox.ask(LeaveRoomMessage { user_id }).await
    }
}

impl Message for LeaveRoomMessage {
    type Response = ();
}

#[postbox::async_trait]
impl Handler<LeaveRoomMessage> for RoomActor {
    type Response = ();

    async fn handle(
        &mut self,
        message: LeaveRoomMessage,
        _context: &mut Context<Self>,
    ) -> Self::Response {
        std::todo!();
    }
}
