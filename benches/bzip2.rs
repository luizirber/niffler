use std::io::Seek;
use std::io::Write;

mod share;

use share::{read_all_stream, write_all_data, BASIC_FILE, BZIP_FILE};

use criterion::{criterion_group, criterion_main, Criterion};

fn read_in_ram(c: &mut Criterion) {
    let mut g = c.benchmark_group("Bzip reads");

    g.bench_function("niffler", |b| {
        b.iter(|| read_all_stream(niffler::get_reader(Box::new(BZIP_FILE)).unwrap().0))
    });
    g.bench_function("bzip2", |b| {
        b.iter(|| read_all_stream(Box::new(bzip2::read::BzDecoder::new(BZIP_FILE))))
    });
}

fn write_in_ram(c: &mut Criterion) {
    let mut out = std::io::Cursor::new(Vec::new());

    let mut g = c.benchmark_group("Bzip write");

    g.bench_function("niffler", |b| {
        b.iter(|| {
            write_all_data(
                niffler::get_writer(
                    Box::new(&mut out),
                    niffler::compression::Format::Bzip,
                    niffler::level::Level::One,
                )
                .unwrap(),
                BASIC_FILE,
            )
        })
    });

    g.bench_function("bzip2", |b| {
        b.iter(|| {
            write_all_data(
                Box::new(bzip2::write::BzEncoder::new(
                    &mut out,
                    bzip2::Compression::fast(),
                )),
                BASIC_FILE,
            )
        })
    });
}

fn read_on_disk(c: &mut Criterion) {
    let mut compress_file = tempfile::NamedTempFile::new().unwrap();

    // fill file
    {
        let wfile = compress_file.reopen().unwrap();
        let mut writer = niffler::get_writer(
            Box::new(wfile),
            niffler::compression::Format::Bzip,
            niffler::level::Level::One,
        )
        .unwrap();

        for _ in 0..(8 * 1024) {
            writer.write(&[42]).unwrap();
        }

        writer.flush().unwrap();
    }

    let mut g = c.benchmark_group("Bzip reads on disk");

    g.bench_function("niffler", |b| {
        b.iter(|| {
            compress_file.seek(std::io::SeekFrom::Start(0)).unwrap();

            read_all_stream(
                niffler::get_reader(Box::new(compress_file.as_file()))
                    .unwrap()
                    .0,
            );
        })
    });

    g.bench_function("bzip2", |b| {
        b.iter(|| {
            compress_file.seek(std::io::SeekFrom::Start(0)).unwrap();

            read_all_stream(Box::new(bzip2::read::BzDecoder::new(
                compress_file.as_file(),
            )));
        })
    });
}

fn write_on_disk(c: &mut Criterion) {
    let compress_file = tempfile::NamedTempFile::new().unwrap();

    let mut g = c.benchmark_group("Bzip2 write on disk");

    g.bench_function("niffler", |b| {
        b.iter(|| {
            let wfile = compress_file.reopen().unwrap();
            let mut writer = niffler::get_writer(
                Box::new(wfile),
                niffler::compression::Format::Bzip,
                niffler::level::Level::One,
            )
            .unwrap();

            for _ in 0..(8 * 1024) {
                writer.write(&[42]).unwrap();
            }
        })
    });

    g.bench_function("bzip2", |b| {
        b.iter(|| {
            let wfile = compress_file.reopen().unwrap();
            let mut writer = bzip2::write::BzEncoder::new(wfile, bzip2::Compression::fast());

            for _ in 0..(8 * 1024) {
                writer.write(&[42]).unwrap();
            }
        })
    });
}

fn setup(c: &mut Criterion) {
    read_in_ram(c);
    write_in_ram(c);

    read_on_disk(c);
    write_on_disk(c);
}

criterion_group!(benches, setup);
criterion_main!(benches);
