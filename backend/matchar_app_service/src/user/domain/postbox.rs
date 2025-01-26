use postbox::{Actor, Postbox};
use refinement::{Storable, UserId};

pub struct UserPostbox {
    pub(crate) postbox: Postbox<User>,
}

pub struct User {
    pub(crate) user_id: UserId,
    pub(crate) name: String,
    pub(crate) emitter: tunnel::Emitter,
}

impl UserPostbox {
    #[inline]
    pub(crate) const fn user_id(&self) -> UserId {
        UserId::new_unchecked(self.postbox.id())
    }
}

impl From<Postbox<User>> for UserPostbox {
    #[inline]
    fn from(postbox: Postbox<User>) -> Self {
        Self { postbox }
    }
}

impl Storable for UserPostbox {
    type Id = UserId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.user_id()
    }
}

impl User {
    pub fn new(user_id: UserId, name: String, emitter: tunnel::Emitter) -> Self {
        Self {
            user_id,
            name,
            emitter,
        }
    }
}

impl Actor for User {
    //
}
