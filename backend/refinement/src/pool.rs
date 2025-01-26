use std::collections::HashMap;

pub trait Item {
    type Id: Copy + Eq + std::hash::Hash;

    fn id(&self) -> Self::Id;
}

pub struct Pool<T: Item> {
    max_len: usize,
    map: HashMap<T::Id, T>,
}

impl<T: Item> Pool<T> {
    #[inline]
    pub fn new(max_len: usize) -> Self {
        Self {
            max_len,
            map: HashMap::new(),
        }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(max_len: usize, iter: I) -> Option<Self> {
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
    pub fn contains_key(&self, id: T::Id) -> bool {
        self.map.contains_key(&id)
    }

    pub fn insert<I: Into<T>>(&mut self, item: I) -> bool {
        let can_insert = !self.is_full();
        if can_insert {
            let item = item.into();
            self.map.insert(item.id(), item);
        }

        can_insert
    }

    #[inline]
    pub fn remove(&mut self, id: T::Id) {
        self.map.remove(&id);
    }

    #[inline]
    pub fn get(&self, id: T::Id) -> Option<&T> {
        self.map.get(&id)
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &T::Id> {
        self.map.keys()
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.map.values()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&T::Id, &T)> {
        self.map.iter()
    }
}
