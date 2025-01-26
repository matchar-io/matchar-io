use super::EnteredEvent;
use crate::{channel::domain::Channel, user::emit_event::Event};
use postbox::{Context, Handler, Message};
use refinement::ChannelId;

#[derive(Clone, Serialize)]
struct Payload {
    channel_id: ChannelId,
    count: usize,
}

impl Event for Payload {
    const TYPE: &'static str = "channel.entered";
}

#[postbox::async_trait]
impl Handler<EnteredEvent> for Channel {
    type Executed = <EnteredEvent as Message>::Executed;

    async fn on_execute(&mut self, event: EnteredEvent, _: &mut Context<Self>) -> Self::Executed {
        let event = Payload::from(event);
        for lobby_user in self.lobby_users.values() {
            let _ = lobby_user.event().emit_event(event.clone());
        }

        Ok(())
    }
}

impl From<EnteredEvent> for Payload {
    #[inline]
    fn from(EnteredEvent { channel_id, count }: EnteredEvent) -> Self {
        Payload { channel_id, count }
    }
}
