/*
Copyright (c) 2020 Pierre Marijon <pmarijon@mpi-inf.mpg.de>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Originally from https://github.com/natir/yacrd/blob/3fc6ef8b5b51256f0c4bc45b8056167acf34fa58/src/file.rs
*/

use std::io::Read;

use niffler;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Benches format detection to be sure modification in the code didn't create performance regression or to test if we have a better method in future
const GZIP_FILE: &'static [u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf3, 0x54, 0xcf, 0x55, 0x48, 0xce,
    0xcf, 0x2d, 0x28, 0x4a, 0x2d, 0x2e, 0x56, 0xc8, 0xcc, 0x53, 0x48, 0xaf, 0xca, 0x2c, 0xe0, 0x02,
    0x00, 0x45, 0x7c, 0xf4, 0x10, 0x15, 0x00, 0x00, 0x00,
];

const BZIP_FILE: &'static [u8] = &[
    0x42, 0x5a, 0x68, 0x39, 0x31, 0x41, 0x59, 0x26, 0x53, 0x59, 0xcc, 0x51, 0x35, 0x90, 0x00, 0x00,
    0x03, 0x5d, 0x80, 0x00, 0x10, 0x40, 0x80, 0x10, 0x00, 0x00, 0x20, 0x1a, 0x23, 0xd8, 0x10, 0x20,
    0x00, 0x22, 0x9a, 0x32, 0x68, 0xf4, 0x8f, 0x28, 0x53, 0x00, 0x04, 0xd3, 0x20, 0x19, 0xf6, 0xa6,
    0xc5, 0x90, 0x48, 0xb5, 0x72, 0x92, 0xf8, 0xbb, 0x92, 0x29, 0xc2, 0x84, 0x86, 0x62, 0x89, 0xac,
    0x80, 0x00,
];

const LZMA_FILE: &'static [u8] = &[
    0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00, 0x00, 0x04, 0xe6, 0xd6, 0xb4, 0x46, 0x02, 0x00, 0x21, 0x01,
    0x16, 0x00, 0x00, 0x00, 0x74, 0x2f, 0xe5, 0xa3, 0x01, 0x00, 0x14, 0x49, 0x27, 0x6d, 0x20, 0x63,
    0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x6c, 0x7a, 0x6d, 0x61, 0x0a,
    0x00, 0x00, 0x00, 0x00, 0x4d, 0x4e, 0x36, 0xfd, 0xff, 0x2e, 0x12, 0xc6, 0x00, 0x01, 0x2d, 0x15,
    0x2f, 0x0b, 0x71, 0x6d, 0x1f, 0xb6, 0xf3, 0x7d, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x59, 0x5a,
];

fn detect_format(c: &mut Criterion) {
    let mut g = c.benchmark_group("Format detection");

    g.bench_function("gzip", |b| {
        b.iter(|| black_box(niffler::compression::read_compression(Box::new(GZIP_FILE))))
    });
    g.bench_function("bzip", |b| {
        b.iter(|| black_box(niffler::compression::read_compression(Box::new(BZIP_FILE))))
    });
    g.bench_function("lzma", |b| {
        b.iter(|| black_box(niffler::compression::read_compression(Box::new(LZMA_FILE))))
    });
}

// Benches file reading
fn read_all_stream<'a>(stream: Box<dyn std::io::Read + 'a>) {
    for b in stream.bytes() {
        black_box(b).unwrap();
    }
}

fn reads_in_ram(c: &mut Criterion) {
    // bench short in ram gzip stream
    {
        let mut g = c.benchmark_group("Gzip reads");

        g.bench_function("niffler", |b| {
            b.iter(|| read_all_stream(niffler::get_reader(Box::new(GZIP_FILE)).unwrap().0))
        });
        g.bench_function("flate2", |b| {
            b.iter(|| read_all_stream(Box::new(flate2::read::GzDecoder::new(GZIP_FILE))))
        });
    }

    // bench short in ram bzip2 stream
    {
        let mut g = c.benchmark_group("Bzip2 reads");

        g.bench_function("niffler", |b| {
            b.iter(|| read_all_stream(niffler::get_reader(Box::new(BZIP_FILE)).unwrap().0))
        });
        g.bench_function("bzip2", |b| {
            b.iter(|| read_all_stream(Box::new(bzip2::read::BzDecoder::new(BZIP_FILE))))
        });
    }

    // bench short in ram lzma stream
    {
        let mut g = c.benchmark_group("LZMA reads");

        g.bench_function("niffler", |b| {
            b.iter(|| read_all_stream(niffler::get_reader(Box::new(LZMA_FILE)).unwrap().0))
        });
        g.bench_function("xz2", |b| {
            b.iter(|| read_all_stream(Box::new(xz2::read::XzDecoder::new(LZMA_FILE))))
        });
    }
}

fn setup(c: &mut Criterion) {
    detect_format(c);
    reads_in_ram(c);
}

criterion_group!(benches, setup);
criterion_main!(benches);
