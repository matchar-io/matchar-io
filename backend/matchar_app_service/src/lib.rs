#![allow(async_fn_in_trait)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate thiserror;

pub mod auth;
pub mod channel;
pub mod game;
pub mod me;
pub mod room;
pub mod session;
pub mod user;

mod common;
