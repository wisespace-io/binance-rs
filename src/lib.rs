//#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]
pub mod config;
pub mod error;
pub mod model;
mod parser;
pub mod rest;

pub use rest::Binance;
