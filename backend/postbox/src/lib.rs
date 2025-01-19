#[macro_use]
extern crate tokio;

#[macro_use]
extern crate async_trait;

mod actor;
mod office;
mod postbox;
mod registry;

pub use actor::*;
pub use async_trait::async_trait;
pub use office::*;
pub use postbox::*;
pub use registry::*;

// #[inline]
// pub fn spawn<A: Actor>(actor: A) -> Postbox<A> {
//     Postbox::create(actor)
// }
