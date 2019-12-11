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

/* crates use */
use cfg_if::cfg_if;
use enum_primitive::{enum_from_primitive, enum_from_primitive_impl, enum_from_primitive_impl_ty};
use flate2;
use thiserror::Error;

enum_from_primitive! {
    #[repr(u64)]
    #[derive(Debug, PartialEq)]
    pub enum CompressionFormat {
        Gzip = 0x1F8B,
        Bzip = 0x425A,
        Lzma = 0x00FD_377A_585A,
        No,
    }
}

#[derive(Debug, Error)]
pub enum NifflerError {
    #[error("Feature disabled, enabled it during compilation")]
    FeatureDisabled,

    #[error("File is too short, less than five bytes")]
    FileTooShort,
}

#[derive(Debug, PartialEq)]
pub enum CompressionLevel {
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

impl Into<u32> for CompressionLevel {
    fn into(self) -> u32 {
        match self {
            CompressionLevel::One => 1,
            CompressionLevel::Two => 2,
            CompressionLevel::Three => 3,
            CompressionLevel::Four => 4,
            CompressionLevel::Five => 5,
            CompressionLevel::Six => 6,
            CompressionLevel::Seven => 7,
            CompressionLevel::Eight => 8,
            CompressionLevel::Nine => 9,
        }
    }
}

impl Into<flate2::Compression> for CompressionLevel {
    fn into(self) -> flate2::Compression {
        match self {
            CompressionLevel::One => flate2::Compression::new(1),
            CompressionLevel::Two => flate2::Compression::new(2),
            CompressionLevel::Three => flate2::Compression::new(3),
            CompressionLevel::Four => flate2::Compression::new(4),
            CompressionLevel::Five => flate2::Compression::new(5),
            CompressionLevel::Six => flate2::Compression::new(6),
            CompressionLevel::Seven => flate2::Compression::new(7),
            CompressionLevel::Eight => flate2::Compression::new(8),
            CompressionLevel::Nine => flate2::Compression::new(9),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "bz2")] {
    use bzip2;

    impl Into<bzip2::Compression> for CompressionLevel {
            fn into(self) -> bzip2::Compression {
        match self {
            CompressionLevel::One   => bzip2::Compression::Fastest,
            CompressionLevel::Two   => bzip2::Compression::Default,
            CompressionLevel::Three => bzip2::Compression::Default,
            CompressionLevel::Four  => bzip2::Compression::Default,
            CompressionLevel::Five  => bzip2::Compression::Default,
            CompressionLevel::Six   => bzip2::Compression::Default,
            CompressionLevel::Seven => bzip2::Compression::Default,
            CompressionLevel::Eight => bzip2::Compression::Default,
            CompressionLevel::Nine  => bzip2::Compression::Best,
        }
            }
    }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level2u32() {
        let tmp: u32 = CompressionLevel::One.into();
        assert_eq!(tmp, 1);
    }

    #[test]
    fn level2flate2() {
        let tmp: flate2::Compression = CompressionLevel::One.into();
        assert_eq!(tmp, flate2::Compression::new(1));
    }

    #[test]
    #[cfg(feature = "bz2")]
    fn level2bzip2() {
        let tmp: bzip2::Compression = CompressionLevel::One.into();
        assert!(match tmp {
            bzip2::Compression::Fastest => true,
            _ => false,
        });
    }
}
