#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

#[macro_use]
extern crate error_chain;

mod client;
pub mod errors;
pub mod util;

pub mod model;

pub mod account;
pub mod api;
pub mod config;
pub mod general;
pub mod market;
pub mod userstream;
pub mod websockets;

pub mod futures;
