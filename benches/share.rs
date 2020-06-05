use std::io::Read;
use std::io::Write;

#[allow(dead_code)]
pub const BASIC_FILE: &'static [u8] = b"I'm not compressed";

#[allow(dead_code)]
pub const GZIP_FILE: &'static [u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf3, 0x54, 0xcf, 0x55, 0x48, 0xce,
    0xcf, 0x2d, 0x28, 0x4a, 0x2d, 0x2e, 0x56, 0xc8, 0xcc, 0x53, 0x48, 0xaf, 0xca, 0x2c, 0xe0, 0x02,
    0x00, 0x45, 0x7c, 0xf4, 0x10, 0x15, 0x00, 0x00, 0x00,
];

#[allow(dead_code)]
pub const BZIP_FILE: &'static [u8] = &[
    0x42, 0x5a, 0x68, 0x39, 0x31, 0x41, 0x59, 0x26, 0x53, 0x59, 0xcc, 0x51, 0x35, 0x90, 0x00, 0x00,
    0x03, 0x5d, 0x80, 0x00, 0x10, 0x40, 0x80, 0x10, 0x00, 0x00, 0x20, 0x1a, 0x23, 0xd8, 0x10, 0x20,
    0x00, 0x22, 0x9a, 0x32, 0x68, 0xf4, 0x8f, 0x28, 0x53, 0x00, 0x04, 0xd3, 0x20, 0x19, 0xf6, 0xa6,
    0xc5, 0x90, 0x48, 0xb5, 0x72, 0x92, 0xf8, 0xbb, 0x92, 0x29, 0xc2, 0x84, 0x86, 0x62, 0x89, 0xac,
    0x80, 0x00,
];

#[allow(dead_code)]
pub const LZMA_FILE: &'static [u8] = &[
    0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00, 0x00, 0x04, 0xe6, 0xd6, 0xb4, 0x46, 0x02, 0x00, 0x21, 0x01,
    0x16, 0x00, 0x00, 0x00, 0x74, 0x2f, 0xe5, 0xa3, 0x01, 0x00, 0x14, 0x49, 0x27, 0x6d, 0x20, 0x63,
    0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x6c, 0x7a, 0x6d, 0x61, 0x0a,
    0x00, 0x00, 0x00, 0x00, 0x4d, 0x4e, 0x36, 0xfd, 0xff, 0x2e, 0x12, 0xc6, 0x00, 0x01, 0x2d, 0x15,
    0x2f, 0x0b, 0x71, 0x6d, 0x1f, 0xb6, 0xf3, 0x7d, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x59, 0x5a,
];

#[allow(dead_code)]
pub fn read_all_stream<'a>(stream: Box<dyn std::io::Read + 'a>) {
    for b in stream.bytes() {
        criterion::black_box(b).unwrap();
    }
}

#[allow(dead_code)]
pub fn write_all_data<'a>(mut stream: Box<dyn std::io::Write + 'a>, data: &[u8]) {
    criterion::black_box(stream.write(data)).unwrap();
}