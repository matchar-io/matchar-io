#![allow(async_fn_in_trait)]

#[macro_use]
extern crate serde;

pub mod channel;
pub mod game;
pub mod room;
pub mod user;

mod command;
mod event;
mod pool;
mod postbox;
