mod share;

use share::{BASIC_FILE, BGZIP_FILE, BZIP_FILE, GZIP_FILE, LZMA_FILE};

use niffler;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn detect_format(c: &mut Criterion) {
    let mut g = c.benchmark_group("Format detection");

    g.bench_function("unflate", |b| {
        b.iter(|| black_box(niffler::sniff(Box::new(BASIC_FILE))))
    });

    g.bench_function("gzip", |b| {
        b.iter(|| black_box(niffler::sniff(Box::new(GZIP_FILE))))
    });
    g.bench_function("bzip", |b| {
        b.iter(|| black_box(niffler::sniff(Box::new(BZIP_FILE))))
    });
    g.bench_function("lzma", |b| {
        b.iter(|| black_box(niffler::sniff(Box::new(LZMA_FILE))))
    });
    g.bench_function("bgzip", |b| {
        b.iter(|| {
            black_box(niffler::seek::sniff(Box::new(std::io::Cursor::new(
                BGZIP_FILE,
            ))))
        })
    });
}

criterion_group!(benches, detect_format);
criterion_main!(benches);
