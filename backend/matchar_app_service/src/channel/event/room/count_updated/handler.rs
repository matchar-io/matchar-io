use super::RoomCountUpdatedEvent;
use crate::{channel::domain::Channel, user::emit_event::Event};
use postbox::{Context, Handler, Message};
use refinement::RoomId;

#[derive(Clone, Serialize)]
struct Payload {
    room: Room,
}

#[derive(Clone, Serialize)]
struct Room {
    room_id: RoomId,
    count: usize,
}

impl Event for Payload {
    const TYPE: &'static str = "channel.room.count.updated";
}

impl From<RoomCountUpdatedEvent> for Payload {
    #[inline]
    fn from(RoomCountUpdatedEvent { room }: RoomCountUpdatedEvent) -> Self {
        Payload {
            room: Room {
                room_id: room.room_id,
                count: room.count,
            },
        }
    }
}

#[postbox::async_trait]
impl Handler<RoomCountUpdatedEvent> for Channel {
    type Executed = <RoomCountUpdatedEvent as Message>::Executed;

    async fn on_execute(
        &mut self,
        event: RoomCountUpdatedEvent,
        _: &mut Context<Self>,
    ) -> Self::Executed {
        let event = Payload::from(event);
        for lobby_user in self.lobby_users.values() {
            let _ = lobby_user.event().emit_event(event.clone());
        }

        Ok(())
    }
}
