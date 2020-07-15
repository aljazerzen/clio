#![forbid(unsafe_code)]
#![warn(clippy::all)]
//! clio is a library for parsing CLI file names.
//!
//! It implemts the standard unix convetions of when the file name is "-" then sending the
//! data to stdin/stdout as apropriate
//! ```
//! // a cat replacement
//! fn main() -> clio::Result<()> {
//!   let args: Vec<_> = std::env::args_os().collect();
//!   let mut input = clio::Input::new(&args[1])?;
//!   std::io::copy(&mut input, &mut std::io::stdout())?;
//!   Ok(())
//! }
//! ```
//!

mod error;
mod input;
mod output;

pub use crate::error::Error;
pub use crate::error::Result;
pub use crate::input::Input;
pub use crate::output::Output;
