use crate::{FromRequestParts, Parts};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type DynamicExtension = Box<dyn Any + Sync + Send>;

pub(crate) struct Extensions {
    map: HashMap<TypeId, ExtensionEntry<DynamicExtension>>,
}

pub struct Extension<T>(pub T);

#[derive(Clone)]
struct ExtensionEntry<T>(T);

impl Extensions {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub(crate) fn insert<T: Sync + Send + 'static>(&mut self, extension: T) {
        self.map
            .insert(TypeId::of::<T>(), ExtensionEntry(Box::new(extension)));
    }
}

impl<T> FromRequestParts for Extension<T>
where
    T: Clone + 'static,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts) -> Result<Self, ()> {
        parts
            .extensions
            .map
            .get(&TypeId::of::<T>())
            .and_then(|ExtensionEntry(extension)| extension.downcast_ref::<T>())
            .map(|extension| Extension(extension.clone()))
            .ok_or(())
    }
}
