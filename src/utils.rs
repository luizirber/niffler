/* standard use */
use std::io;

/* project use */
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
