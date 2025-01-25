#![allow(clippy::unnecessary_wraps)]

pub use crate::basic::compression::Format;

pub(crate) fn bytes2type(bytes: [u8; 5]) -> Format {
    match bytes {
        [0x1f, 0x8b, ..] => Format::Gzip,
        [0x42, 0x5a, ..] => Format::Bzip,
        [0xfd, 0x37, 0x7a, 0x58, 0x5a] => Format::Lzma,
        [0x28, 0xb5, 0x2f, 0xfd, ..] => Format::Zstd,
        _ => Format::No,
    }
}

impl_format!(
    gz,
    "gz",
    crate::basic::compression::Format::Gzip,
    flate2::write::GzEncoder::new,
    flate2::read::MultiGzDecoder::new,
    std::io::Read | Send,
    std::io::Write | Send,
    crate::basic::compression::Format
);

impl_format!(
    bz2,
    "bz2",
    crate::basic::compression::Format::Bzip,
    bzip2::write::BzEncoder::new,
    bzip2::read::MultiBzDecoder::new,
    std::io::Read | Send,
    std::io::Write | Send,
    crate::basic::compression::Format
);

impl_format!(
    lzma,
    "lzma",
    crate::basic::compression::Format::Lzma,
    liblzma::write::XzEncoder::new,
    liblzma::read::XzDecoder::new,
    std::io::Read | Send,
    std::io::Write | Send,
    crate::basic::compression::Format
);

pub mod zstd {
    /* standard use */
    use std::io;

    /* project use */
    use crate::error::Error;

    /* project use */
    use crate::level::Level;

    use super::Format;

    #[cfg(feature = "zstd")]
    pub(crate) fn encoder<'a>(
        out: Box<dyn io::Write + Send + 'a>,
        level: Level,
    ) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
        Ok(Box::new(
            zstd::stream::write::Encoder::new(out, level.into())?.auto_finish(),
        ))
    }

    #[cfg(feature = "zstd")]
    pub(crate) fn decoder<'a>(
        inp: Box<dyn io::Read + Send + 'a>,
    ) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
        Ok((
            Box::new(zstd::stream::read::Decoder::new(inp)?),
            Format::Zstd,
        ))
    }

    #[cfg(not(feature = "zstd"))]
    pub(crate) fn encoder<'a>(
        _: Box<dyn io::Write + Send + 'a>,
        _: Level,
    ) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
        Err(Error::FeatureDisabled)
    }

    #[cfg(not(feature = "zstd"))]
    pub(crate) fn decoder<'a>(
        _: Box<dyn io::Read + Send + 'a>,
    ) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
        Err(Error::FeatureDisabled)
    }
}
