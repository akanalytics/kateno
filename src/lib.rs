// #![cfg_attr(debug_assertions, allow(dead_code))]
#![allow(dead_code)]
#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![allow(mixed_script_confusables)]

mod error;
mod selection;
mod text_parser;
mod util;
mod graveyard;

pub mod prelude;
