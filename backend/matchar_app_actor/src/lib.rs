#![allow(async_fn_in_trait)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate thiserror;

pub mod channel;
pub mod game;
pub mod message;
pub mod room;
pub mod user;

pub use message::*;

mod command;
mod event;
mod pool;
mod postbox;
