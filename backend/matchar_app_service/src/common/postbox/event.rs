use postbox::Postbox;

#[derive(Clone)]
pub struct Event<A>
where
    A: postbox::Actor,
{
    pub(crate) postbox: Postbox<A>,
}

impl<A> std::ops::Deref for Event<A>
where
    A: postbox::Actor,
{
    type Target = Postbox<A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.postbox
    }
}

impl<A> From<Postbox<A>> for Event<A>
where
    A: postbox::Actor,
{
    #[inline]
    fn from(postbox: Postbox<A>) -> Self {
        Self { postbox }
    }
}
