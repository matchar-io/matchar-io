use postbox::{Actor, Postbox};

#[derive(Clone)]
pub struct Event<A>
where
    A: Actor,
{
    postbox: Postbox<A>,
}

impl<A> Event<A>
where
    A: Actor,
{
    pub fn new(postbox: Postbox<A>) -> Self {
        Self { postbox }
    }
}

impl<A> std::ops::Deref for Event<A>
where
    A: Actor,
{
    type Target = Postbox<A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.postbox
    }
}

impl<A> From<Postbox<A>> for Event<A>
where
    A: Actor,
{
    #[inline]
    fn from(postbox: Postbox<A>) -> Self {
        Self { postbox }
    }
}
