pub mod command;
pub mod event;
pub mod pool;

pub use command::*;
pub use event::*;
pub use pool::*;

#[derive(Clone)]
pub struct Postbox<A>
where
    A: postbox::Actor,
{
    postbox: postbox::Postbox<A>,
}

impl<A> Postbox<A>
where
    A: postbox::Actor,
{
    #[inline]
    pub fn new(postbox: postbox::Postbox<A>) -> Self {
        Self { postbox }
    }

    #[inline]
    pub(crate) fn id(&self) -> A::Id {
        self.postbox.id().into()
    }

    #[inline]
    pub fn command(&self) -> Command<A> {
        self.postbox.clone().into()
    }

    #[inline]
    pub(crate) fn event(&self) -> Event<A> {
        self.postbox.clone().into()
    }
}

impl<A> From<postbox::Postbox<A>> for Postbox<A>
where
    A: postbox::Actor,
{
    #[inline]
    fn from(postbox: postbox::Postbox<A>) -> Self {
        Self { postbox }
    }
}
