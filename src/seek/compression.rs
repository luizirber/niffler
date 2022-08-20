/* standard use */
use std::io;

/* Some trait definition */
pub trait ReadSeek: io::Read + io::Seek {}

impl<T> ReadSeek for T where T: io::Read + io::Seek {}

pub trait WriteSeek: io::Write + io::Seek {}

impl<T> WriteSeek for T where T: io::Write + io::Seek {}

/// `Format` represent a compression format of a file. Currently BGzip are supported.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Format {
    BGzip,
    No,
}

pub(crate) fn bytes2type(bytes: [u8; 17]) -> Format {
    match bytes {
        [0x1F, 0x8B, 0x8, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0xFF, 0x6, 0x0, 0x42, 0x43, 0x2, 0x0, 0x0] => {
            Format::BGzip
        }
        _ => Format::No,
    }
}
