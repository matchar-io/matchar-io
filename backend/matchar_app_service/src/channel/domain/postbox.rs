use crate::{room::RoomPostbox, user::UserPostbox};
use postbox::{Actor, Postbox};
use refinement::{ChannelId, RoomId, UserId};
use std::collections::HashMap;

pub struct ChannelPostbox {
    pub(crate) postbox: Postbox<Channel>,
}

pub struct Channel {
    pub(crate) channel_id: ChannelId,
    pub(crate) name: String,
    pub(crate) users: HashMap<UserId, UserPostbox>,
    pub(crate) rooms: HashMap<RoomId, RoomPostbox>,
}

impl From<Postbox<Channel>> for ChannelPostbox {
    #[inline]
    fn from(postbox: Postbox<Channel>) -> Self {
        Self { postbox }
    }
}

impl Actor for Channel {
    //
}
