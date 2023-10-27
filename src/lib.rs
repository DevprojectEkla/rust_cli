//!rust_cli is a simple crate to easily manage cli application developpment providing all the basic
//!stuff you need with a certain level of abstraction which makes it easier to use rust
//!functionality with a bit less verbosity.
pub mod choice;
pub mod cli_args;
pub mod cli_utils;
pub use crate::choice::Choice;
pub use crate::cli_args::ParseArguments;
#[cfg(test)]
mod choice_tests;



   

