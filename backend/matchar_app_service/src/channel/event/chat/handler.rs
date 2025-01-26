use super::ChatEvent;
use crate::{channel::domain::Channel, user::emit_event::Event};
use postbox::{Context, Handler, Message};
use refinement::{UserId, UserName};

#[derive(Clone, Serialize)]
struct Payload {
    user: User,
    message: String,
}

#[derive(Clone, Serialize)]
struct User {
    user_id: UserId,
    name: UserName,
}

impl Event for Payload {
    const TYPE: &'static str = "channel.chat";
}

#[postbox::async_trait]
impl Handler<ChatEvent> for Channel {
    type Executed = <ChatEvent as Message>::Executed;

    async fn on_execute(&mut self, event: ChatEvent, _: &mut Context<Self>) -> Self::Executed {
        let event = Payload::from(event);
        for lobby_user in self.lobby_users.values() {
            let _ = lobby_user.event().emit_event(event.clone());
        }

        Ok(())
    }
}

impl From<ChatEvent> for Payload {
    #[inline]
    fn from(ChatEvent { user, message }: ChatEvent) -> Self {
        Payload {
            user: User {
                user_id: user.user_id,
                name: user.name,
            },
            message,
        }
    }
}
