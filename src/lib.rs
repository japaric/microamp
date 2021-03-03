#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![doc(include = "../README.md")]
#![feature(external_doc)]
#![deny(missing_docs)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![no_std]

pub use microamp_macros::shared;

mod cfail;
#[doc(hidden)]
pub mod export;
