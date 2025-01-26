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

impl From<EnteredEvent> for Payload {
    #[inline]
    fn from(EnteredEvent { user, room_id }: EnteredEvent) -> Self {
        Payload {
            room_id: room_id,
            user: User {
                user_id: user.user_id,
                name: user.name,
            },
        }
    }
}

#[postbox::async_trait]
impl Handler<EnteredEvent> for Room {
    type Executed = <EnteredEvent as Message>::Executed;

    async fn on_execute(&mut self, event: EnteredEvent, _: &mut Context<Self>) -> Self::Executed {
        let event = Payload::from(event);
        for user in self.players.values() {
            let _ = user.event().emit_event(event.clone());
        }

        Ok(())
    }
}
