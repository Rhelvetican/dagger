//! # Dagger
//!
//! The Balatro mod manager API.

mod dir;
mod error;
mod ext;
mod git;
mod lua;
mod specification;

pub mod cli;

type BoxedStr = Box<str>;
pub use error::*;
