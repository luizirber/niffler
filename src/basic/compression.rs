#![allow(clippy::unnecessary_wraps)]

/* standard use */
use std::io;

/* crates use */
use cfg_if::cfg_if;

/* project use */
use crate::error::Error;

/* backward compatibility, can remove on 3.x */
pub use crate::level::Level;

/* Format detection enum */
/// `Format` represent a compression format of a file. Currently Gzip, Bzip, Lzma or No are supported.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
    Gzip,
    Bzip,
    Lzma,
    Zstd,
    No,
}

impl Format {
    #[allow(non_upper_case_globals)]
    pub const Xz: Format = Format::Lzma;
}

pub(crate) fn get_first_five<'a>(
    mut in_stream: Box<dyn io::Read + 'a>,
) -> Result<([u8; 5], Box<dyn io::Read + 'a>), Error> {
    let mut buf = [0u8; 5];
    match in_stream.read_exact(&mut buf) {
        Ok(()) => Ok((buf, in_stream)),
        Err(_) => Err(Error::FileTooShort),
    }
}

pub(crate) fn bytes2type(bytes: [u8; 5]) -> Format {
    match bytes {
        [0x1f, 0x8b, ..] => Format::Gzip,
        [0x42, 0x5a, ..] => Format::Bzip,
        [0x28, 0xb5, 0x2f, 0xfd, ..] => Format::Zstd,
        [0xfd, 0x37, 0x7a, 0x58, 0x5a] => Format::Lzma,
        _ => Format::No,
    }
}

cfg_if! {
    if #[cfg(feature = "gz")] {
        pub(crate) fn new_gz_encoder<'a>(out: Box<dyn io::Write  + 'a>, level: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Ok(Box::new(flate2::write::GzEncoder::new(
        out,
        level.into(),
            )))
        }

        pub(crate) fn new_gz_decoder<'a>(
            inp: Box<dyn io::Read  + 'a>,
        ) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Ok((
        Box::new(flate2::read::MultiGzDecoder::new(inp)),
        Format::Gzip,
            ))
        }
    } else {
        pub(crate) fn new_gz_encoder<'a>(_: Box<dyn io::Write  + 'a>, _: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Err(Error::FeatureDisabled)
        }

        pub(crate) fn new_gz_decoder<'a>(_: Box<dyn io::Read  + 'a>) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Err(Error::FeatureDisabled)
        }
    }
}

cfg_if! {
    if #[cfg(feature = "bz2")] {
        pub(crate) fn new_bz2_encoder<'a>(out: Box<dyn io::Write  + 'a>, level: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Ok(Box::new(bzip2::write::BzEncoder::new(
                out,
                level.into(),
            )))
        }

        pub(crate) fn new_bz2_decoder<'a>(
            inp: Box<dyn io::Read  + 'a>,
        ) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Ok((
                Box::new(bzip2::read::BzDecoder::new(inp)),
                Format::Bzip,
            ))
        }
    } else {
        pub(crate) fn new_bz2_encoder<'a>(_: Box<dyn io::Write  + 'a>, _: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Err(Error::FeatureDisabled)
        }

        pub(crate) fn new_bz2_decoder<'a>(_: Box<dyn io::Read  + 'a>) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Err(Error::FeatureDisabled)
        }
    }
}

cfg_if! {
    if #[cfg(feature = "lzma")] {
    pub(crate) fn new_lzma_encoder<'a>(out: Box<dyn io::Write  + 'a>, level: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Ok(Box::new(xz2::write::XzEncoder::new(out, level.into())))
    }

    pub(crate) fn new_lzma_decoder<'a>(
            inp: Box<dyn io::Read  + 'a>,
    ) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Ok((
        Box::new(xz2::read::XzDecoder::new(inp)),
        Format::Lzma,
            ))
    }
    } else {
    pub(crate) fn new_lzma_encoder<'a>(_: Box<dyn io::Write  + 'a>, _: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Err(Error::FeatureDisabled)
    }

    pub(crate) fn new_lzma_decoder<'a>(_: Box<dyn io::Read  + 'a>) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Err(Error::FeatureDisabled)
    }
    }
}

cfg_if! {
    if #[cfg(feature = "zstd")] {
        pub(crate) fn new_zstd_encoder<'a>(out: Box<dyn io::Write  + 'a>, level: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Ok(Box::new(zstd::stream::write::Encoder::new(
                        out,
                        level.into(),
            )?.auto_finish()))
        }

        pub(crate) fn new_zstd_decoder<'a>(
            inp: Box<dyn io::Read  + 'a>,
        ) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Ok((Box::new(zstd::stream::read::Decoder::new(inp)?),
                         Format::Zstd,
            ))
        }
    } else {
        pub(crate) fn new_zstd_encoder<'a>(_: Box<dyn io::Write  + 'a>, _: Level) -> Result<Box<dyn io::Write  + 'a>, Error> {
            Err(Error::FeatureDisabled)
        }

        pub(crate) fn new_zstd_decoder<'a>(_: Box<dyn io::Read  + 'a>) -> Result<(Box<dyn io::Read  + 'a>, Format), Error> {
            Err(Error::FeatureDisabled)
        }
    }
}
