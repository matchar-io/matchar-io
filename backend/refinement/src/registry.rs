use std::collections::HashMap;

pub trait Storable {
    type Id: Copy + Eq + std::hash::Hash;

    fn id(&self) -> Self::Id;
}

pub struct Registry<Item: Storable> {
    max_len: usize,
    items: HashMap<Item::Id, Item>,
}

impl<Item: Storable> Registry<Item> {
    #[inline]
    pub fn new(max_len: usize) -> Self {
        Self {
            max_len,
            items: HashMap::new(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.items.len() == self.max_len
    }

    pub fn insert<I: Into<Item>>(&mut self, item: I) -> bool {
        let can_insert = !self.is_full();
        if can_insert {
            let item = item.into();
            self.items.insert(item.id(), item);
        }

        can_insert
    }

    #[inline]
    pub fn remove(&mut self, id: Item::Id) {
        self.items.remove(&id);
    }

    #[inline]
    pub fn get(&self, id: Item::Id) -> Option<&Item> {
        self.items.get(&id)
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &Item::Id> {
        self.items.keys()
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Item> {
        self.items.values()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Item::Id, &Item)> {
        self.items.iter()
    }
}
