use crate::{FromRequest, Request};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type DynamicExtension = Box<dyn AnyClone + Send>;

trait AnyClone: Any {
    fn clone_box(&self) -> DynamicExtension;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub struct Extensions {
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
        Box::new(self.clone())
    }

    #[inline]
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
        let type_id = TypeId::of::<T>();
        let extension = Box::new(AnyCloneExtension(extension));
        self.map.insert(type_id, ExtensionEntry(extension));
    }

    pub(crate) fn clone_all(&self) -> Self {
        let mut map = HashMap::new();
        for (type_id, extension) in &self.map {
            map.insert(*type_id, ExtensionEntry(extension.0.clone_box()));
        }

        Self { map }
    }
}

impl<T> FromRequest for Extension<T>
where
    T: Clone + 'static,
{
    type Rejection = ();

    async fn from_request(request: &mut Request) -> Result<Self, ()> {
        let ExtensionEntry(extension) = request.extensions.map.get(&TypeId::of::<T>()).ok_or(())?;
        let extension = extension.clone_box().into_any();
        let extension: Box<AnyCloneExtension<T>> = extension.downcast().or_else(|_| Err(()))?;

        Ok(Extension(extension.0))
    }
}
