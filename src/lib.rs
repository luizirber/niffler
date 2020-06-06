/*
Copyright (c) 2018 Pierre Marijon <pierre.marijon@inria.fr>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Originally from https://github.com/natir/yacrd/blob/3fc6ef8b5b51256f0c4bc45b8056167acf34fa58/src/file.rs
*/

//! # niffler
//! Simple and transparent support for compressed files.
//!
//! This library provides two main features:
//! - sniffs out compression formats from input files and return a
//! Read trait object ready for consumption.
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
//!
//! let mut buffer = Vec::new();
//!
//! {
//!   let mut writer = niffler::get_writer(Box::new(&mut buffer), compression::Format::Gzip, compression::Level::Nine)?;
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
//! # Ok(())
//! # }
//! ```

/* standard use */
use std::io;
use std::io::Read;
use std::path::Path;

/* extern crate use */
use enum_primitive::FromPrimitive;

/* crates section */
pub mod compression;
pub mod error;

pub use crate::error::Error;

/// Finds out what is the compression format for a stream based on magic numbers
/// (the first few bytes of the stream).
///
/// Return the stream and the compression format detected.
///
/// # Example
/// ```
/// # fn main() -> Result<(), niffler::Error> {
///
/// let data = vec![
///         0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf3, 0x54, 0xcf, 0x55,
///         0x48, 0xce, 0xcf, 0x2d, 0x28, 0x4a, 0x2d, 0x2e, 0x56, 0xc8, 0xcc, 0x53, 0x48, 0xaf,
///         0xca, 0x2c, 0xe0, 0x02, 0x00, 0x45, 0x7c, 0xf4, 0x10, 0x15, 0x00, 0x00, 0x00
///         ];
///
/// let (mut reader, compression) = niffler::sniff(Box::new(&data[..]))?;
///
/// let mut contents = Vec::new();
/// reader.read_to_end(&mut contents).expect("Error durring file reading");
///
/// assert_eq!(compression, niffler::compression::Format::Gzip);
/// assert_eq!(contents, data);
/// # Ok(())
/// # }
/// ```
pub fn sniff<'a>(
    in_stream: Box<dyn io::Read + 'a>,
) -> Result<(Box<dyn io::Read + 'a>, compression::Format), Error> {
    let (first_bytes, in_stream) = compression::get_first_five(in_stream)?;

    let mut five_bit_val: u64 = 0;
    for (i, item) in first_bytes.iter().enumerate().take(5) {
        five_bit_val |= (u64::from(*item)) << (8 * (4 - i));
    }

    if compression::Format::from_u64(five_bit_val) == Some(compression::Format::Lzma) {
        let cursor = io::Cursor::new(first_bytes);
        return Ok((Box::new(cursor.chain(in_stream)), compression::Format::Lzma));
    }

    let mut two_bit_val: u64 = 0;
    for (i, item) in first_bytes.iter().enumerate().take(2) {
        two_bit_val |= (u64::from(*item)) << (8 * (1 - i));
    }

    let cursor = io::Cursor::new(first_bytes);
    match compression::Format::from_u64(two_bit_val) {
        e @ Some(compression::Format::Gzip) | e @ Some(compression::Format::Bzip) => {
            Ok((Box::new(cursor.chain(in_stream)), e.unwrap()))
        }
        _ => Ok((Box::new(cursor.chain(in_stream)), compression::Format::No)),
    }
}

/// Create a readable stream that can be read transparently even if the original stream is compress.
/// Also returns the compression type of the original stream.
///
/// # Example
/// ```
/// use niffler::{Error, get_reader};
/// # fn main() -> Result<(), Error> {
///
/// let probably_compress_stream = std::io::Cursor::new(vec![
///         0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf3, 0x54, 0xcf, 0x55,
///         0x48, 0xce, 0xcf, 0x2d, 0x28, 0x4a, 0x2d, 0x2e, 0x56, 0xc8, 0xcc, 0x53, 0x48, 0xaf,
///         0xca, 0x2c, 0xe0, 0x02, 0x00, 0x45, 0x7c, 0xf4, 0x10, 0x15, 0x00, 0x00, 0x00
///         ]);
///
/// let (mut reader, compression) = niffler::get_reader(Box::new(probably_compress_stream))?;
///
/// let mut contents = String::new();
/// reader.read_to_string(&mut contents).expect("Error durring file reading");
///
/// assert_eq!(compression, niffler::compression::Format::Gzip);
/// assert_eq!(contents, "I'm compress in gzip\n");
/// # Ok(())
/// # }
/// ```
pub fn get_reader<'a>(
    in_stream: Box<dyn io::Read + 'a>,
) -> Result<(Box<dyn io::Read + 'a>, compression::Format), Error> {
    // check compression
    let (in_stream, compression) = sniff(in_stream)?;

    // return readable and compression status
    match compression {
        compression::Format::Gzip => Ok((
            Box::new(flate2::read::GzDecoder::new(in_stream)),
            compression::Format::Gzip,
        )),
        compression::Format::Bzip => compression::new_bz2_decoder(in_stream),
        compression::Format::Lzma => compression::new_lzma_decoder(in_stream),
        compression::Format::No => Ok((in_stream, compression::Format::No)),
    }
}

/// Create a new writable stream with the given compression format and level.
///
/// # Example
/// ```
/// use std::io::Read;
/// use niffler::{Error, get_writer, compression};
/// # fn main() -> Result<(), Error> {
///
/// let mut buffer = vec![];
/// {
///   let mut writer = niffler::get_writer(Box::new(&mut buffer), compression::Format::Gzip, compression::Level::One)?;
///   writer.write_all("I'm compress in gzip\n".as_bytes())?
/// }
///
/// let mut contents = Vec::new();
/// buffer.as_slice().read_to_end(&mut contents)?;
///
/// assert_eq!(contents, vec![
///         0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xff, 0xf3, 0x54, 0xcf, 0x55,
///         0x48, 0xce, 0xcf, 0x2d, 0x28, 0x4a, 0x2d, 0x2e, 0x56, 0xc8, 0xcc, 0x53, 0x48, 0xaf,
///         0xca, 0x2c, 0xe0, 0x02, 0x00, 0x45, 0x7c, 0xf4, 0x10, 0x15, 0x00, 0x00, 0x00
///         ]);
/// # Ok(())
/// # }
/// ```

pub fn get_writer<'a>(
    out_stream: Box<dyn io::Write + 'a>,
    format: compression::Format,
    level: compression::Level,
) -> Result<Box<dyn io::Write + 'a>, Error> {
    match format {
        compression::Format::Gzip => Ok(Box::new(flate2::write::GzEncoder::new(
            out_stream,
            level.into(),
        ))),
        compression::Format::Bzip => compression::new_bz2_encoder(out_stream, level),
        compression::Format::Lzma => compression::new_lzma_encoder(out_stream, level),
        compression::Format::No => Ok(Box::new(out_stream)),
    }
}

/// Open a possibly compressed file and decompress it transparently.
/// ```
/// use niffler::{Error, compression};
/// # fn main() -> Result<(), Error> {
///
/// # let file = tempfile::NamedTempFile::new()?;
///
/// # {
/// #   let mut writer = niffler::to_path(file.path(), compression::Format::Gzip, compression::Level::Nine)?;
/// #   writer.write_all(b"hello")?;
/// # }
///
/// let (mut reader, format) = niffler::from_path(file.path())?;
///
/// let mut contents = vec![];
/// reader.read_to_end(&mut contents);
/// # assert_eq!(&contents, b"hello");
///
/// # Ok(())
/// # }
/// ```
pub fn from_path<'a, P: AsRef<Path>>(
    path: P,
) -> Result<(Box<dyn io::Read + 'a>, compression::Format), Error> {
    let readable = io::BufReader::new(std::fs::File::open(path)?);
    get_reader(Box::new(readable))
}

/// Create a file with specific compression format.
/// ```
/// use niffler::{Error, compression};
/// # fn main() -> Result<(), Error> {
///
/// # let file = tempfile::NamedTempFile::new()?;
///
/// # {
/// let mut writer = niffler::to_path(file.path(), compression::Format::Gzip, compression::Level::Nine)?;
/// writer.write_all(b"hello")?;
/// # }
///
/// # let (mut reader, format) = niffler::from_path(&file.path())?;
/// # let mut contents = vec![];
/// # reader.read_to_end(&mut contents)?;
/// # assert_eq!(&contents, b"hello");
/// # Ok(())
/// # }
/// ```
pub fn to_path<'a, P: AsRef<Path>>(
    path: P,
    format: compression::Format,
    level: compression::Level,
) -> Result<Box<dyn io::Write + 'a>, Error> {
    let writable = io::BufWriter::new(std::fs::File::create(path)?);
    get_writer(Box::new(writable), format, level)
}

#[cfg(test)]
mod test {

    use super::*;

    pub(crate) const SHORT_FILE: &'static [u8] = &[0o037, 0o213, 0o0, 0o0];
    pub(crate) const GZIP_FILE: &'static [u8] = &[0o037, 0o213, 0o0, 0o0, 0o0];
    pub(crate) const BZIP_FILE: &'static [u8] = &[0o102, 0o132, 0o0, 0o0, 0o0];
    pub(crate) const LZMA_FILE: &'static [u8] = &[0o375, 0o067, 0o172, 0o130, 0o132];
    pub(crate) const LOREM_IPSUM: &'static [u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut ultricies scelerisque diam, a scelerisque enim sagittis at.";

    mod compress_uncompress {
        use super::*;
        use tempfile::NamedTempFile;

        #[test]
        fn no_compression() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer = get_writer(
                    Box::new(wfile),
                    compression::Format::No,
                    compression::Level::One,
                )
                .unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::No);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }

        #[test]
        fn gzip() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer = get_writer(
                    Box::new(wfile),
                    compression::Format::Gzip,
                    compression::Level::Six,
                )
                .unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::Gzip);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }

        #[test]
        #[cfg(not(feature = "bz2"))]
        fn no_bzip2_feature() {
            assert!(
                get_writer(
                    Box::new(vec![]),
                    compression::Format::Bzip,
                    compression::Level::Six
                )
                .is_err(),
                "bz2 disabled, this assertion should fail"
            );

            assert!(
                get_reader(Box::new(&BZIP_FILE[..])).is_err(),
                "bz2 disabled, this assertion should fail"
            );
        }

        #[cfg(feature = "bz2")]
        #[test]
        fn bzip() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer = get_writer(
                    Box::new(wfile),
                    compression::Format::Bzip,
                    compression::Level::Six,
                )
                .unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::Bzip);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }

        #[test]
        #[cfg(not(feature = "lzma"))]
        fn no_lzma_feature() {
            assert!(
                get_writer(
                    Box::new(vec![]),
                    compression::Format::Lzma,
                    compression::Level::Six
                )
                .is_err(),
                "lzma disabled, this assertion should fail"
            );

            assert!(
                get_reader(Box::new(&LZMA_FILE[..])).is_err(),
                "lzma disabled, this assertion should fail"
            );
        }

        #[cfg(feature = "lzma")]
        #[test]
        fn lzma() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer = get_writer(
                    Box::new(wfile),
                    compression::Format::Lzma,
                    compression::Level::Six,
                )
                .unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::Lzma);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }
    }

    mod compression_format_detection {
        use super::*;

        #[test]
        fn gzip() {
            let (_, compression) = sniff(Box::new(GZIP_FILE)).expect("Error in read file");
            assert_eq!(compression, compression::Format::Gzip);
        }

        #[test]
        fn bzip() {
            let (_, compression) = sniff(Box::new(BZIP_FILE)).expect("Error in read file");
            assert_eq!(compression, compression::Format::Bzip);
        }

        #[test]
        fn lzma() {
            let (_, compression) = sniff(Box::new(LZMA_FILE)).expect("Error in read file");
            assert_eq!(compression, compression::Format::Lzma);
        }

        #[test]
        fn too_short() {
            let result = sniff(Box::new(SHORT_FILE));
            assert!(result.is_err());
        }

        #[test]
        fn no_compression() {
            let (_, compression) = sniff(Box::new(LOREM_IPSUM)).expect("Error in read file");
            assert_eq!(compression, compression::Format::No);
        }
    }
}
