#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#[macro_use]
extern crate error_chain;

extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
extern crate serde_json;

extern crate tungstenite;
extern crate url;

#[macro_use]
extern crate serde_derive;

mod util;
mod client;
pub mod errors;

pub mod model;

pub mod api;
pub mod general;
pub mod account;
pub mod market;
pub mod userstream;
pub mod websockets;
