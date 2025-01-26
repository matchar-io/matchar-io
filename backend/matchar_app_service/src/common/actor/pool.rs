use super::Postbox;
use postbox::Actor;
use std::collections::HashMap;

pub struct Pool<A: Actor> {
    max_len: usize,
    map: HashMap<A::Id, Postbox<A>>,
}

impl<A: Actor> Pool<A> {
    #[inline]
    pub fn new(max_len: usize) -> Self {
        Self {
            max_len,
            map: HashMap::new(),
        }
    }

    pub fn from_iter<I>(max_len: usize, iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Postbox<A>>,
    {
        let mut map = HashMap::new();
        for item in iter {
            map.insert(item.id(), item);
        }

        if map.len() > max_len {
            None
        } else {
            Some(Self { max_len, map })
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.map.len() == self.max_len
    }

    #[inline]
    pub fn contains_key(&self, id: A::Id) -> bool {
        self.map.contains_key(&id)
    }

    pub fn insert<I: Into<Postbox<A>>>(&mut self, item: I) -> bool {
        let can_insert = !self.is_full();
        if can_insert {
            let item = item.into();
            self.map.insert(item.id(), item);
        }

        can_insert
    }

    #[inline]
    pub fn remove(&mut self, id: A::Id) {
        self.map.remove(&id);
    }

    #[inline]
    pub fn get(&self, id: A::Id) -> Option<&Postbox<A>> {
        self.map.get(&id)
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &A::Id> {
        self.map.keys()
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Postbox<A>> {
        self.map.values()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&A::Id, &Postbox<A>)> {
        self.map.iter()
    }
}
