use crate::user::{User, UserPostbox};
use postbox::{Context, Handler, Message, PostboxError, PostboxResult};

pub struct Chat {
    message: String,
}

pub enum ChatError {
    Postbox(PostboxError),
    Emitter(tunnel::EmitterError),
}

impl UserPostbox {
    pub fn chat(&self, message: String) -> PostboxResult<()> {
        self.postbox.tell(Chat { message })
    }
}

impl Message for Chat {
    type Response = Result<(), ChatError>;
}

#[postbox::async_trait]
impl Handler<Chat> for User {
    type Response = <Chat as Message>::Response;

    async fn handle(
        &mut self,
        Chat { message }: Chat,
        _context: &mut Context<Self>,
    ) -> Self::Response {
        #[derive(Serialize)]
        struct Payload {
            message: String,
        }

        self.emitter
            .emit(Payload { message })
            .await
            .map_err(ChatError::Emitter)?;

        Ok(())
    }
}
