#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::ref_patterns)]
#![allow(clippy::todo)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::pub_use)]
#![allow(clippy::mod_module_files)]
#![allow(async_fn_in_trait)]

#[cfg(feature = "postgres")]
pub mod postgres;