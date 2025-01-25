#![allow(clippy::unnecessary_wraps)]

/* backward compatibility, can remove on 3.x */
pub use crate::level::Level;

/* Format detection enum */
/// `Format` represent a compression format of a file. Currently Gzip, Bzip, Lzma, Zstd or No are supported.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

pub(crate) fn bytes2type(bytes: [u8; 5]) -> Format {
    match bytes {
        [0x1f, 0x8b, ..] => Format::Gzip,
        [0x42, 0x5a, ..] => Format::Bzip,
        [0x28, 0xb5, 0x2f, 0xfd, ..] => Format::Zstd,
        [0xfd, 0x37, 0x7a, 0x58, 0x5a] => Format::Lzma,
        _ => Format::No,
    }
}

impl_format!(
    gz,
    "gz",
    crate::basic::compression::Format::Gzip,
    flate2::write::GzEncoder::new,
    flate2::read::MultiGzDecoder::new,
    std::io::Read,
    std::io::Write,
    crate::basic::compression::Format
);

impl_format!(
    bz2,
    "bz2",
    crate::basic::compression::Format::Bzip,
    bzip2::write::BzEncoder::new,
    bzip2::read::MultiBzDecoder::new,
    std::io::Read,
    std::io::Write,
    crate::basic::compression::Format
);

impl_format!(
    lzma,
    "lzma",
    crate::basic::compression::Format::Lzma,
    liblzma::write::XzEncoder::new,
    liblzma::read::XzDecoder::new,
    std::io::Read,
    std::io::Write,
    crate::basic::compression::Format
);

pub mod zstd {
    /* standard use */
    use std::io;

    /* project use */
    use crate::error::Error;

    use super::{Format, Level};

    #[cfg(feature = "zstd")]
    pub(crate) fn encoder<'a>(
        out: Box<dyn io::Write + 'a>,
        level: Level,
    ) -> Result<Box<dyn io::Write + 'a>, Error> {
        Ok(Box::new(
            zstd::stream::write::Encoder::new(out, level.into())?.auto_finish(),
        ))
    }

    #[cfg(feature = "zstd")]
    pub(crate) fn decoder<'a>(
        inp: Box<dyn io::Read + 'a>,
    ) -> Result<(Box<dyn io::Read + 'a>, Format), Error> {
        Ok((
            Box::new(zstd::stream::read::Decoder::new(inp)?),
            Format::Zstd,
        ))
    }

    #[cfg(not(feature = "zstd"))]
    pub(crate) fn encoder<'a>(
        _: Box<dyn io::Write + 'a>,
        _: Level,
    ) -> Result<Box<dyn io::Write + 'a>, Error> {
        Err(Error::FeatureDisabled)
    }

    #[cfg(not(feature = "zstd"))]
    pub(crate) fn decoder<'a>(
        _: Box<dyn io::Read + 'a>,
    ) -> Result<(Box<dyn io::Read + 'a>, Format), Error> {
        Err(Error::FeatureDisabled)
    }
}
