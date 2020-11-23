/* standard use */
use std::io;

/* crates use */
use cfg_if::cfg_if;
use enum_primitive::{
    enum_from_primitive, enum_from_primitive_impl, enum_from_primitive_impl_ty, FromPrimitive,
};

/* project use */
use crate::error::Error;
use crate::level::Level;

/* Format detection enum */
enum_from_primitive! {
    #[repr(u64)]
    /// `Format` represent a compression format of a file. Currently Gzip, Bzip, Lzma or No are supported.
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Format {
        Gzip = 0x1F8B,
        Bzip = 0x425A,
        Lzma = 0x00FD_377A_585A,
        No,
    }
}

pub(crate) fn get_first_five<'a>(
    mut in_stream: Box<dyn io::Read + Send + 'a>,
) -> Result<([u8; 5], Box<dyn io::Read + Send + 'a>), Error> {
    let mut buf = [0u8; 5];
    match in_stream.read_exact(&mut buf) {
        Ok(()) => Ok((buf, in_stream)),
        Err(_) => Err(Error::FileTooShort),
    }
}

pub(crate) fn bytes2type(bytes: [u8; 5]) -> Format {
    let mut five_bit_val: u64 = 0;
    for (i, item) in bytes.iter().enumerate().take(5) {
        five_bit_val |= (u64::from(*item)) << (8 * (4 - i));
    }

    if Format::from_u64(five_bit_val) == Some(Format::Lzma) {
        return Format::Lzma;
    }

    let mut two_bit_val: u64 = 0;
    for (i, item) in bytes.iter().enumerate().take(2) {
        two_bit_val |= (u64::from(*item)) << (8 * (1 - i));
    }

    match Format::from_u64(two_bit_val) {
        e @ Some(Format::Gzip) | e @ Some(Format::Bzip) => e.unwrap(),
        _ => Format::No,
    }
}

cfg_if! {
    if #[cfg(feature = "gz")] {
        pub(crate) fn new_gz_encoder<'a>(out: Box<dyn io::Write + Send + 'a>, level: Level) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
            Ok(Box::new(flate2::write::GzEncoder::new(
        out,
        level.into(),
            )))
        }

        pub(crate) fn new_gz_decoder<'a>(
            inp: Box<dyn io::Read + Send + 'a>,
        ) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
            Ok((
        Box::new(flate2::read::MultiGzDecoder::new(inp)),
        Format::Gzip,
            ))
        }
    } else {
        pub(crate) fn new_gz_encoder<'a>(_: Box<dyn io::Write + Send + 'a>, _: Level) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
            Err(Error::FeatureDisabled)
        }

        pub(crate) fn new_gz_decoder<'a>(_: Box<dyn io::Read + Send + 'a>) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
            Err(Error::FeatureDisabled)
        }
    }
}

cfg_if! {
    if #[cfg(feature = "bz2")] {
        pub(crate) fn new_bz2_encoder<'a>(out: Box<dyn io::Write + Send + 'a>, level: Level) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
            Ok(Box::new(bzip2::write::BzEncoder::new(
                out,
                level.into(),
            )))
        }

        pub(crate) fn new_bz2_decoder<'a>(
            inp: Box<dyn io::Read + Send + 'a>,
        ) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
            Ok((
                Box::new(bzip2::read::BzDecoder::new(inp)),
                Format::Bzip,
            ))
        }
    } else {
        pub(crate) fn new_bz2_encoder<'a>(_: Box<dyn io::Write + Send + 'a>, _: Level) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
            Err(Error::FeatureDisabled)
        }

        pub(crate) fn new_bz2_decoder<'a>(_: Box<dyn io::Read + Send + 'a>) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
            Err(Error::FeatureDisabled)
        }
    }
}

cfg_if! {
    if #[cfg(feature = "lzma")] {
    pub(crate) fn new_lzma_encoder<'a>(out: Box<dyn io::Write + Send + 'a>, level: Level) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
            Ok(Box::new(xz2::write::XzEncoder::new(out, level.into())))
    }

    pub(crate) fn new_lzma_decoder<'a>(
            inp: Box<dyn io::Read + Send + 'a>,
    ) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
            Ok((
        Box::new(xz2::read::XzDecoder::new(inp)),
        Format::Lzma,
            ))
    }
    } else {
    pub(crate) fn new_lzma_encoder<'a>(_: Box<dyn io::Write + Send + 'a>, _: Level) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
            Err(Error::FeatureDisabled)
    }

    pub(crate) fn new_lzma_decoder<'a>(_: Box<dyn io::Read + Send + 'a>) -> Result<(Box<dyn io::Read + Send + 'a>, Format), Error> {
            Err(Error::FeatureDisabled)
    }
    }
}
