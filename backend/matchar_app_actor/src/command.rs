use postbox::{Actor, Postbox};
use refinement::UserId;

#[derive(Clone)]
pub struct Command<A: Actor> {
    postbox: Postbox<A>,
}

impl<A> Command<A>
where
    A: Actor,
{
    pub fn new(postbox: Postbox<A>) -> Self {
        Self { postbox }
    }

    #[inline]
    pub const fn user_id(&self) -> UserId {
        UserId::new_unchecked(self.postbox.id())
    }
}

impl<A> std::ops::Deref for Command<A>
where
    A: Actor,
{
    type Target = Postbox<A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.postbox
    }
}

impl<A> From<Postbox<A>> for Command<A>
where
    A: Actor,
{
    #[inline]
    fn from(postbox: Postbox<A>) -> Self {
        Self { postbox }
    }
}
