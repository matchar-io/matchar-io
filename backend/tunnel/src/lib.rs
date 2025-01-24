#![allow(unused)]

pub mod emitter;
pub mod message;
pub mod receiver;

pub use emitter::*;
pub use message::*;
pub use receiver::*;

pub fn channel<T>(capacity: usize) -> (Emitter, Receiver) {
    let (tx, rx) = tokio::sync::mpsc::channel(capacity);

    let emitter = Emitter::new(tx);
    let receiver = Receiver::new(rx, capacity);

    (emitter, receiver)
}
