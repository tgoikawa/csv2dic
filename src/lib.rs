#![cfg(test)]
extern crate test_case;
#[macro_use]
extern crate validator_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod csv_src;
mod dest;
mod error;
mod template;
mod word;

pub use crate::error::*;
use crate::word::*;

pub fn convert() {}
