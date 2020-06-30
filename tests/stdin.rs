use std::io::{Seek, SeekFrom, Write};

use assert_cmd::prelude::*;
use predicates::str::contains;

#[cfg(not(tarpaulin))] // Tarpaulin interacts weirdly with escargot
#[cfg(feature = "gz")]
#[test]
fn test_stdin_gz() {
    let input: &[u8] = &[
        0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 255, 203, 72, 205, 201, 201, 7, 0, 134, 166, 16, 54, 5, 0,
        0, 0,
    ];
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(input).unwrap();
    file.flush().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    escargot::CargoBuild::new()
        .example("stdin_pipe")
        .current_release()
        .current_target()
        .run()
        .unwrap()
        .command()
        .stdin(file.into_file())
        .assert()
        .success()
        .stdout(contains("hello"))
        .stdout(contains("Gzip"));
}

#[cfg(not(tarpaulin))] // Tarpaulin interacts weirdly with escargot
#[cfg(feature = "bz2")]
#[test]
fn test_stdin_bz2() {
    let input: &[u8] = &[
        0x42, 0x5a, 0x68, 0x39, 0x31, 0x41, 0x59, 0x26, 0x53, 0x59, 0xc1, 0xc0, 0x80, 0xe2, 0x00,
        0x00, 0x01, 0x41, 0x00, 0x00, 0x10, 0x02, 0x44, 0xa0, 0x00, 0x30, 0xcd, 0x00, 0xc3, 0x46,
        0x29, 0x97, 0x17, 0x72, 0x45, 0x38, 0x50, 0x90, 0xc1, 0xc0, 0x80, 0xe2,
    ];
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(input).unwrap();
    file.flush().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    escargot::CargoBuild::new()
        .example("stdin_pipe")
        .current_release()
        .current_target()
        .features("bz2")
        .run()
        .unwrap()
        .command()
        .stdin(file.into_file())
        .assert()
        .success()
        .stdout(contains("hello"))
        .stdout(contains("Bzip"));
}

#[cfg(not(tarpaulin))] // Tarpaulin interacts weirdly with escargot
#[cfg(feature = "lzma")]
#[test]
fn test_stdin_lzma() {
    let input: &[u8] = &[
        0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00, 0x00, 0x04, 0xe6, 0xd6, 0xb4, 0x46, 0x02, 0x00, 0x21,
        0x01, 0x16, 0x00, 0x00, 0x00, 0x74, 0x2f, 0xe5, 0xa3, 0x01, 0x00, 0x05, 0x68, 0x65, 0x6c,
        0x6c, 0x6f, 0x0a, 0x00, 0x00, 0x00, 0xa5, 0x60, 0x97, 0xf1, 0x94, 0xf6, 0xfd, 0xe0, 0x00,
        0x01, 0x1e, 0x06, 0xc1, 0x2f, 0xa4, 0x1d, 0x1f, 0xb6, 0xf3, 0x7d, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x04, 0x59, 0x5a,
    ];
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(input).unwrap();
    file.flush().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    escargot::CargoBuild::new()
        .example("stdin_pipe")
        .current_release()
        .current_target()
        .features("lzma")
        .run()
        .unwrap()
        .command()
        .stdin(file.into_file())
        .assert()
        .success()
        .stdout(contains("hello"))
        .stdout(contains("Lzma"));
}
