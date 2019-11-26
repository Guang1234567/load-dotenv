//! This is a small procedural macro to load your `.env` file at compile time. That way you can use
//! [`std::env!`][] to load environment variables and fail the build if a variable is missing.
//!
//! All it does is call the [dotenv](https://crates.io/crates/dotenv) crate.
//!
//! # Example
//!
//! `.env` file:
//!
//! ```text
//! KEY=value
//! ```
//!
//! Rust:
//!
//! ```
//! use load_dotenv::load_dotenv;
//!
//! load_dotenv!();
//!
//! fn main() {
//!     assert_eq!("value", env!("KEY"));
//! }
//! ```
//!
//! [`std::env!`]: https://doc.rust-lang.org/std/macro.env.html

#![doc(html_root_url = "https://docs.rs/load-dotenv/0.1.1")]

extern crate proc_macro;

use quote::quote;

/// Load the `.env` file and panic if the file is missing.
#[proc_macro]
pub fn load_dotenv(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv::dotenv().expect("Failed to load .env file");

    (quote! {}).into()
}

/// Load the `.env.xxx` file and panic if the file is missing.
#[proc_macro]
pub fn load_dotenv_from_filename(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _input: String = input.to_string();
    let _input: &str = _input.trim_matches('"');
    dotenv::from_filename(_input).expect(format!("Failed to load {} file", _input).as_str());

    (quote! {}).into()
}

/// Load the `.env` file but don't panic if the file is missing.
#[proc_macro]
pub fn try_load_dotenv(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv::dotenv().ok();

    (quote! {}).into()
}
