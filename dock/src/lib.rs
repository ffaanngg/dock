//! 🛳 The simple, fast and powerful command line parser

#![warn(rust_2018_idioms, clippy::pedantic, missing_docs)]

pub mod core;

pub use crate::core::*;

pub use ansi_term::Color;
