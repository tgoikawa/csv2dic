#![cfg(test)]
extern crate test_case;
#[macro_use]
extern crate validator_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod comark_dest;
mod csv_src;
mod error;
mod word;

pub use crate::error::*;
use crate::word::*;

pub fn convert() {}
