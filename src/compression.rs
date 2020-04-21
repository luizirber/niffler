/*
Copyright (c) 2018 Pierre Marijon <pmarijon@mpi-inf.mpg.de>

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
 */

/* standard use */
use std::io;
use std::io::Read;

/* crates use */
use cfg_if::cfg_if;
use enum_primitive::{
    enum_from_primitive, enum_from_primitive_impl, enum_from_primitive_impl_ty, FromPrimitive,
};
use flate2;

use crate::error::Error;

enum_from_primitive! {
    #[repr(u64)]
    /// `Format` represent a compression format of a file. Currently Gzip, Bzip, Lzma or No are supported.
    #[derive(Debug, PartialEq)]
    pub enum Format {
        Gzip = 0x1F8B,
        Bzip = 0x425A,
        Lzma = 0x00FD_377A_585A,
        No,
    }
}

/// `Level` represent the compression level this value is include between 1 to 9.
/// 1 optimize the compression time,
/// 9 optimize the size of the output.
///
/// For bzip2:
///  - `One` is convert in `bzip2::Compression::Fastest`,
///  - `Nine` in `bzip2::Compression::Best`
/// and other value is convert in `bzip2::Compression::Default.
#[derive(Debug, PartialEq)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

fn get_first_five(mut in_stream: Box<dyn io::Read>) -> Result<([u8; 5], Box<dyn io::Read>), Error> {
    let mut buf = [0u8; 5];
    match in_stream.read_exact(&mut buf) {
        Ok(()) => Ok((buf, in_stream)),
        Err(_) => Err(Error::FileTooShort),
    }
}

pub(crate) fn read_compression(
    in_stream: Box<dyn io::Read>,
) -> Result<(Format, Box<dyn io::Read>), Error> {
    let (first_bytes, in_stream) = get_first_five(in_stream)?;

    let mut five_bit_val: u64 = 0;
    for (i, item) in first_bytes.iter().enumerate().take(5) {
        five_bit_val |= (u64::from(*item)) << (8 * (4 - i));
    }

    if Format::from_u64(five_bit_val) == Some(Format::Lzma) {
        let cursor = io::Cursor::new(first_bytes);
        return Ok((Format::Lzma, Box::new(cursor.chain(in_stream))));
    }

    let mut two_bit_val: u64 = 0;
    for (i, item) in first_bytes.iter().enumerate().take(2) {
        two_bit_val |= (u64::from(*item)) << (8 * (1 - i));
    }

    let cursor = io::Cursor::new(first_bytes);
    match Format::from_u64(two_bit_val) {
        e @ Some(Format::Gzip) | e @ Some(Format::Bzip) => {
            Ok((e.unwrap(), Box::new(cursor.chain(in_stream))))
        }
        _ => Ok((Format::No, Box::new(cursor.chain(in_stream)))),
    }
}

cfg_if! {
    if #[cfg(feature = "bz2")] {
        pub(crate) fn new_bz2_encoder(out: Box<dyn io::Write>, level: Level) -> Result<Box<dyn io::Write>, Error> {
            Ok(Box::new(bzip2::write::BzEncoder::new(
                out,
                level.into(),
            )))
        }

        pub(crate) fn new_bz2_decoder(
            inp: Box<dyn io::Read>,
        ) -> Result<(Box<dyn io::Read>, Format), Error> {
            Ok((
                Box::new(bzip2::read::BzDecoder::new(inp)),
                Format::Bzip,
            ))
        }
    } else {
        pub(crate) fn new_bz2_encoder(_: Box<dyn io::Write>, _: Level) -> Result<Box<dyn io::Write>, Error> {
            Err(Error::FeatureDisabled)
        }

        pub(crate) fn new_bz2_decoder(_: Box<dyn io::Read>) -> Result<(Box<dyn io::Read>, Format), Error> {
            Err(Error::FeatureDisabled)
        }
    }
}

cfg_if! {
    if #[cfg(feature = "lzma")] {
      pub(crate) fn new_lzma_encoder(out: Box<dyn io::Write>, level: Level) -> Result<Box<dyn io::Write>, Error> {
          Ok(Box::new(xz2::write::XzEncoder::new(out, level.into())))
      }

      pub(crate) fn new_lzma_decoder(
          inp: Box<dyn io::Read>,
      ) -> Result<(Box<dyn io::Read>, Format), Error> {
          Ok((
              Box::new(xz2::read::XzDecoder::new(inp)),
              Format::Lzma,
          ))
      }
    } else {
      pub(crate) fn new_lzma_encoder(_: Box<dyn io::Write>, _: Level) -> Result<Box<dyn io::Write>, Error> {
          Err(Error::FeatureDisabled)
      }

      pub(crate) fn new_lzma_decoder(_: Box<dyn io::Read>) -> Result<(Box<dyn io::Read>, Format), Error> {
          Err(Error::FeatureDisabled)
      }
    }
}

impl Into<u32> for Level {
    fn into(self) -> u32 {
        match self {
            Level::One => 1,
            Level::Two => 2,
            Level::Three => 3,
            Level::Four => 4,
            Level::Five => 5,
            Level::Six => 6,
            Level::Seven => 7,
            Level::Eight => 8,
            Level::Nine => 9,
        }
    }
}

impl Into<flate2::Compression> for Level {
    fn into(self) -> flate2::Compression {
        match self {
            Level::One => flate2::Compression::new(1),
            Level::Two => flate2::Compression::new(2),
            Level::Three => flate2::Compression::new(3),
            Level::Four => flate2::Compression::new(4),
            Level::Five => flate2::Compression::new(5),
            Level::Six => flate2::Compression::new(6),
            Level::Seven => flate2::Compression::new(7),
            Level::Eight => flate2::Compression::new(8),
            Level::Nine => flate2::Compression::new(9),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "bz2")] {
    impl Into<bzip2::Compression> for Level {
        fn into(self) -> bzip2::Compression {
            match self {
                Level::One   => bzip2::Compression::Fastest,
                Level::Two   => bzip2::Compression::Default,
                Level::Three => bzip2::Compression::Default,
                Level::Four  => bzip2::Compression::Default,
                Level::Five  => bzip2::Compression::Default,
                Level::Six   => bzip2::Compression::Default,
                Level::Seven => bzip2::Compression::Default,
                Level::Eight => bzip2::Compression::Default,
                Level::Nine  => bzip2::Compression::Best,
            }
            }
    }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::{BZIP_FILE, GZIP_FILE, LOREM_IPSUM, LZMA_FILE, SHORT_FILE};

    #[test]
    fn level2u32() {
        let tmp: u32 = Level::One.into();
        assert_eq!(tmp, 1);
    }

    #[test]
    fn level2flate2() {
        let tmp: flate2::Compression = Level::One.into();
        assert_eq!(tmp, flate2::Compression::new(1));
    }

    #[test]
    #[cfg(feature = "bz2")]
    fn level2bzip2() {
        let tmp: bzip2::Compression = Level::One.into();
        assert!(match tmp {
            bzip2::Compression::Fastest => true,
            _ => false,
        });
    }

    mod compression_format_detection {
        use super::*;

        #[test]
        fn gzip() {
            let (compression, _) =
                read_compression(Box::new(GZIP_FILE)).expect("Error in read file");
            assert_eq!(compression, Format::Gzip);
        }

        #[test]
        fn bzip() {
            let (compression, _) =
                read_compression(Box::new(BZIP_FILE)).expect("Error in read file");
            assert_eq!(compression, Format::Bzip);
        }

        #[test]
        fn lzma() {
            let (compression, _) =
                read_compression(Box::new(LZMA_FILE)).expect("Error in read file");
            assert_eq!(compression, Format::Lzma);
        }

        #[test]
        fn too_short() {
            let result = read_compression(Box::new(SHORT_FILE));
            assert!(result.is_err());
        }

        #[test]
        fn no_compression() {
            let (compression, _) =
                read_compression(Box::new(LOREM_IPSUM)).expect("Error in read file");
            assert_eq!(compression, Format::No);
        }
    }
}
