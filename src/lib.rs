//! # niffler
//! Simple and transparent support for compressed files.
//!
//! This library provides two main features:
//! - sniffs out compression formats from input files and return a
//!   `Read` trait object ready for consumption.
//! - Create a Writer initialized with compression ready for writing.
//!
//! The goal is to lower the barrier to open and use a file, especially in
//! bioinformatics workflows.
//!
//! # Example
//!
//! ```rust
//! use niffler::{Error, compression};
//! # fn main() -> Result<(), Error> {
//! # #[cfg(feature = "gz")] {
//! let mut buffer = Vec::new();
//!
//! {
//!   let mut writer = niffler::get_writer(Box::new(&mut buffer), compression::Format::Gzip, niffler::Level::Nine)?;
//!   writer.write_all(b"hello")?;
//! }
//!
//! # assert_eq!(&buffer, &[0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 255, 203, 72, 205, 201, 201, 7, 0, 134, 166, 16, 54, 5, 0, 0, 0]);
//!
//! let (mut reader, compression) = niffler::get_reader(Box::new(&buffer[..]))?;
//!
//! let mut contents = String::new();
//! reader.read_to_string(&mut contents)?;
//!
//! assert_eq!(compression, niffler::compression::Format::Gzip);
//! assert_eq!(contents, "hello");
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! ## Selecting compression formats
//!
//! By default all supported compression formats are enabled.
//! If you're working on systems that don't support them you can disable default
//! features and select the ones you want.
//! For example,
//! currently only `gz` is supported in Webassembly environments
//! (because `niffler` depends on crates that have system dependencies for `bz2` and `lzma` compression),
//! so you can use this in your `Cargo.toml` to select only the `gz` support:
//! ```toml
//! niffler = { version = "2.2.0", default-features = false, features = ["gz"] }
//! ```
//!
//! You can still use `niffler::sniff()` to find what is the compression format,
//! even if any feature is disabled.
//! But if you try to use `niffler::get_reader` for a disabled feature,
//! it will throw a runtime error.
//!
//! ## Backends features
//!
//! The libraries that are used for decompression provide a number of features that can have a significant impact on performance.
//! Here is the list of features available with corresponding backend crates and features name in backend crates:
//! - bz2_tokio -> bzip2 tokio
//! - bz2_static -> bzip2 static
//! - lzma_tokio -> lzma tokio
//! - gz_zlib -> flate2 zlib
//! - gz_zlib-ng-compat -> flate2 zlib-ng-compat
//! - gz_cloudflare_zlib -> flate2 cloudflare_zlib
//! - gz_rust_backend -> flate2 rust_backend
//! - xz_tokio -> xz2 tokio

/* declare mod */
pub mod basic;
pub mod error;
pub mod level;
pub mod seek;
pub mod seeksend;
pub mod send;
pub(crate) mod utils;

/* reexport for convinent usage of niffler */
pub use crate::basic::compression::Format;
pub use crate::basic::*;
pub use crate::error::Error;
pub use crate::level::Level;
