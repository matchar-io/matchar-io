use super::EnteredEvent;
use crate::{room::domain::Room, user::emit_event::Event};
use postbox::{Context, Handler, Message};
use refinement::{RoomId, UserId, UserName};

#[derive(Clone, Serialize)]
struct Payload {
    room_id: RoomId,
    user: User,
}

#[derive(Clone, Serialize)]
struct User {
    user_id: UserId,
    name: UserName,
}

impl Event for Payload {
    const TYPE: &'static str = "room.entered";
}

#[postbox::async_trait]
impl Handler<EnteredEvent> for Room {
    type Response = <EnteredEvent as Message>::Response;

    async fn handle(&mut self, event: EnteredEvent, _: &mut Context<Self>) -> Self::Response {
        let event = Payload::from(event);
        for user in self.players.values() {
            let _ = user.event().emit_event(event.clone());
        }

        Ok(())
    }
}

impl From<EnteredEvent> for Payload {
    #[inline]
    fn from(EnteredEvent { room_id, user }: EnteredEvent) -> Self {
        Payload {
            room_id,
            user: User {
                user_id: user.user_id,
                name: user.name,
            },
        }
    }
}
