use crate::channel::ChannelEvent;
use postbox::{Message, PostboxResult};
use refinement::{UserId, UserName};

#[derive(Clone)]
pub struct ChatEvent {
    pub(crate) user: User,
    pub(crate) message: String,
}

#[derive(Clone)]
pub struct User {
    pub(crate) user_id: UserId,
    pub(crate) name: UserName,
}

impl ChannelEvent {
    pub fn chat(&self, user: User, message: String) -> <ChatEvent as Message>::Executed {
        self.tell(ChatEvent { user, message })
    }
}

impl Message for ChatEvent {
    type Executed = PostboxResult<()>;
}
