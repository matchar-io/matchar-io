#![allow(async_fn_in_trait)]
#![allow(unused)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate thiserror;

pub mod auth;
pub mod me;
pub mod room;
pub mod session;
pub mod user;
