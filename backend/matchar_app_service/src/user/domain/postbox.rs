use postbox::{Actor, Postbox};
use refinement::UserId;

pub struct UserPostbox {
    pub(crate) postbox: Postbox<User>,
}

pub struct User {
    pub(crate) user_id: UserId,
    pub(crate) name: String,
}

impl From<Postbox<User>> for UserPostbox {
    #[inline]
    fn from(postbox: Postbox<User>) -> Self {
        Self { postbox }
    }
}

impl Actor for User {
    //
}
