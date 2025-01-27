use crate::user::UserEvent;
use postbox::{Message, PostboxError, PostboxResult};

pub trait Event: serde::Serialize + Clone + Sync + Send + 'static {
    const TYPE: &'static str;
}

#[derive(Clone)]
pub struct EmitEvent<E> {
    pub(crate) event: E,
}

#[derive(Clone)]
pub enum EmitEventError {
    Postbox(PostboxError),
    Emitter(tunnel::EmitterError),
}

impl UserEvent {
    pub fn emit_event<E: Event>(&self, event: E) -> PostboxResult<()> {
        self.tell(EmitEvent { event })
    }
}

impl<E> Message for EmitEvent<E>
where
    E: Event,
{
    type Executed = Result<(), EmitEventError>;
}
