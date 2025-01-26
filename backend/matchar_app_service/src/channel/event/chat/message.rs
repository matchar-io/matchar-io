use crate::channel::domain::ChannelEvent;
use postbox::{Message, PostboxResult};
use refinement::{UserId, UserName};

pub struct ChatEvent {
    pub(crate) user: User,
    pub(crate) message: String,
}

pub struct User {
    pub(crate) user_id: UserId,
    pub(crate) name: UserName,
}

impl ChannelEvent {
    pub fn chat(&self, user: User, message: String) -> <ChatEvent as Message>::Response {
        self.tell(ChatEvent { user, message })
    }
}

impl Message for ChatEvent {
    type Response = PostboxResult<()>;
}
