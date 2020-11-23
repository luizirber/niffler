/* declare mod */
pub mod compression;

/* standard use */
use std::io;
use std::path::Path;

/* project use */
use crate::error::Error;
use crate::level::Level;

/// Similar as [sniff](crate::sniff) but from seekable format
///
/// Only [compression::Format] are support
pub fn sniff<'a>(
    mut in_stream: Box<dyn compression::ReadSeek + 'a>,
) -> Result<(Box<dyn compression::ReadSeek + 'a>, compression::Format), Error> {
    let first_bytes = compression::get_first_bytes(&mut in_stream)?;

    match compression::bytes2type(first_bytes) {
        e @ compression::Format::BGzip => Ok((in_stream, e)),
        _ => Ok((in_stream, compression::Format::No)),
    }
}

/// Similar as [get_reader](crate::sniff) but from seekable format
///
/// Warning actualy this function work only for not compressed file
pub fn get_reader<'a>(
    in_stream: Box<dyn compression::ReadSeek + 'a>,
) -> Result<(Box<dyn compression::ReadSeek + 'a>, compression::Format), Error> {
    // check compression
    let (in_stream, compression) = sniff(in_stream)?;

    // return readable and compression status
    match compression {
        compression::Format::No => Ok((in_stream, compression::Format::No)),
        _ => Err(Error::FeatureDisabled),
    }
}

/// Similar as [get_writer](crate::get_writer) but from seekable format
///
/// Warning actualy this function work only for not compressed file
pub fn get_writer<'a>(
    out_stream: Box<dyn compression::WriteSeek + 'a>,
    format: compression::Format,
    _level: Level,
) -> Result<Box<dyn compression::WriteSeek + 'a>, Error> {
    match format {
        compression::Format::No => Ok(Box::new(out_stream)),
        _ => Err(Error::FeatureDisabled),
    }
}

/// Similar as [from_path](crate::from_path) but from seekable format
///
/// Warning actualy this function work only for not compressed file
pub fn from_path<'a, P: AsRef<Path>>(
    path: P,
) -> Result<(Box<dyn compression::ReadSeek + 'a>, compression::Format), Error> {
    let readable = io::BufReader::new(std::fs::File::open(path)?);
    get_reader(Box::new(readable))
}

/// Similar as [to_path](crate::to_path) but from seekable format
///
/// Warning actualy this function work only for not compressed file
pub fn to_path<'a, P: AsRef<Path>>(
    path: P,
    format: compression::Format,
    level: Level,
) -> Result<Box<dyn compression::WriteSeek + 'a>, Error> {
    let writable = io::BufWriter::new(std::fs::File::create(path)?);
    get_writer(Box::new(writable), format, level)
}

#[cfg(test)]
mod test {

    use super::*;
    use tempfile::NamedTempFile;

    pub(crate) const SHORT_FILE: &'static [u8] = &[0o037, 0o213, 0o0, 0o0];
    pub(crate) const BGZIP_FILE: &'static [u8] = &[
        0x1F, 0x8B, 0x8, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0xFF, 0x6, 0x0, 0x42, 0x43, 0x2, 0x0, 0x0,
    ];
    pub(crate) const LOREM_IPSUM: &'static [u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut ultricies scelerisque diam, a scelerisque enim sagittis at.";

    mod compression_format_detection {
        use super::*;
        use std::io::Write;

        #[test]
        fn bgzip() {
            let mut ofile = NamedTempFile::new().expect("Can't create tmpfile");

            ofile.write_all(BGZIP_FILE).unwrap();

            let rfile = ofile.reopen().expect("Can't create tmpfile");

            let (_, compression) = sniff(Box::new(rfile)).expect("Error in read file");
            assert_eq!(compression, compression::Format::BGzip);
        }

        #[test]
        fn too_short() {
            let mut ofile = NamedTempFile::new().expect("Can't create tmpfile");

            ofile.write_all(SHORT_FILE).unwrap();

            let rfile = ofile.reopen().expect("Can't create tmpfile");

            let result = sniff(Box::new(rfile));
            assert!(result.is_err());
        }

        #[test]
        fn no_compression() {
            let mut ofile = NamedTempFile::new().expect("Can't create tmpfile");

            ofile.write_all(LOREM_IPSUM).unwrap();

            let rfile = ofile.reopen().expect("Can't create tmpfile");

            let (_, compression) = sniff(Box::new(rfile)).expect("Error in read file");
            assert_eq!(compression, compression::Format::No);
        }
    }
}
