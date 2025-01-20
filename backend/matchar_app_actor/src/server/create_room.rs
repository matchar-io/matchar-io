use super::{ServerActor, ServerPostbox};
use postbox::{Context, Handler, Message, PostboxResult};

struct CreateRoomMessage {
    name: String,
}

impl ServerPostbox {
    pub async fn create_room(&self, name: String) -> PostboxResult<()> {
        self.postbox.ask(CreateRoomMessage { name }).await
    }
}

impl Message for CreateRoomMessage {
    type Response = ();
}

#[postbox::async_trait]
impl Handler<CreateRoomMessage> for ServerActor {
    type Response = ();

    async fn handle(
        &mut self,
        message: CreateRoomMessage,
        _context: &mut Context<Self>,
    ) -> Self::Response {
        std::todo!();
    }
}
