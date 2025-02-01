use crate::{FromRequestParts, Parts};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type DynamicExtension = Box<dyn AnyClone + Send + 'static>;

trait AnyClone: Any {
    fn clone_box(&self) -> DynamicExtension;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub(crate) struct Extensions {
    map: HashMap<TypeId, ExtensionEntry<DynamicExtension>>,
}

pub struct Extension<T>(pub T);

#[derive(Clone)]
struct ExtensionEntry<T>(T);

#[derive(Clone)]
struct AnyCloneExtension<T>(T);

impl<T> AnyClone for AnyCloneExtension<T>
where
    T: Clone + Send + 'static,
{
    fn clone_box(&self) -> DynamicExtension {
        Box::new(AnyCloneExtension(self.0.clone()))
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Extensions {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub(crate) fn insert<T: Clone + Send + 'static>(&mut self, Extension(extension): Extension<T>) {
        self.map.insert(
            TypeId::of::<T>(),
            ExtensionEntry(Box::new(AnyCloneExtension(extension))),
        );
    }

    pub(crate) fn clone_all(&self) -> Self {
        let mut map = HashMap::new();
        for (type_id, extension) in &self.map {
            map.insert(*type_id, ExtensionEntry(extension.0.clone_box()));
        }

        Self { map }
    }
}

impl<T> FromRequestParts for Extension<T>
where
    T: Clone + 'static,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts) -> Result<Self, ()> {
        let ExtensionEntry(extension) = parts.extensions.map.get(&TypeId::of::<T>()).ok_or(())?;
        let extension = extension.clone_box().into_any();
        let extension = extension
            .downcast::<AnyCloneExtension<T>>()
            .ok()
            .ok_or(())?;

        Ok(Extension(extension.0))
    }
}
