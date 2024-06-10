/* standard use */
use std::io;

use crate::error::Error;
use crate::seek::compression::ReadSeek;

pub fn get_first_five<'a, T>(mut in_stream: T) -> Result<([u8; 5], T), Error>
where
    T: io::Read + 'a,
{
    let mut buf = [0u8; 5];
    match in_stream.read_exact(&mut buf) {
        Ok(()) => Ok((buf, in_stream)),
        Err(_) => Err(Error::FileTooShort),
    }
}

pub fn get_first_bytes<'a, T>(in_stream: &mut T) -> Result<[u8; 17], Error>
where
    T: ReadSeek + 'a,
{
    let mut buf = [0u8; 17];

    match in_stream.read_exact(&mut buf) {
        Ok(()) => {
            in_stream.seek(io::SeekFrom::Start(0))?;
            Ok(buf)
        }
        Err(_) => Err(Error::FileTooShort),
    }
}

macro_rules! impl_format {
  ($mod:ident, $feature:literal, $format:path, $encoder:path, $decoder:path, $($rbound:path)|+, $($wbound:path)|+, $all_formats:path) => {

    pub mod $mod {

        #[cfg(feature = $feature)]
        pub(crate) fn encoder<'a>(out: Box<dyn $($wbound + )* 'a>, level: $crate::level::Level) -> Result<Box<dyn $($wbound + )* 'a>, $crate::Error> {
            Ok(Box::new($encoder(out, level.into())))
        }

        #[cfg(feature = $feature)]
        pub(crate) fn decoder<'a>(inp: Box<dyn $($rbound + )* 'a>) -> Result<(Box<dyn $($rbound + )* 'a>, $all_formats), $crate::Error> {
            Ok(( Box::new($decoder(inp)), $format))
        }

        #[cfg(not(feature = $feature))]
        pub(crate) fn encoder<'a>(_: Box<dyn $($wbound +)* 'a>, _: $crate::level::Level) -> Result<Box<dyn $($wbound + )* 'a>, $crate::Error> {
            Err($crate::Error::FeatureDisabled)
        }

        #[cfg(not(feature = $feature))]
        pub(crate) fn decoder<'a>(_: Box<dyn $($rbound +)* 'a>) -> Result<(Box<dyn $($rbound +)* 'a>, $all_formats), $crate::Error> {
            Err($crate::Error::FeatureDisabled)
        }

    }
  }
}
