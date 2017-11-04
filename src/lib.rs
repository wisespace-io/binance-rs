#[macro_use] 
extern crate error_chain;

extern crate ws;
extern crate log;
extern crate hex;
extern crate ring;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use] 
extern crate serde_derive;

mod util;
mod client;
mod errors;
mod model;
mod websockets;

pub mod api;
pub mod general;
pub mod account;
pub mod market;
pub mod userstream;