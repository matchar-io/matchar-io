use crate::user::UserPostbox;
use postbox::{Actor, Postbox};
use refinement::PlayerId;

pub struct PlayerPostbox {
    pub(crate) postbox: Postbox<Player>,
}

pub struct Player {
    pub(crate) player_id: PlayerId,
    pub(crate) user: UserPostbox,
}

impl From<Postbox<Player>> for PlayerPostbox {
    #[inline]
    fn from(postbox: Postbox<Player>) -> Self {
        Self { postbox }
    }
}

impl Actor for Player {
    //
}
