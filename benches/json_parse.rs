/* std use */
use std::io::{Read, Write};

/* crate use */
use criterion::{criterion_group, criterion_main, Criterion};
use reqwest;

/* project use */
mod share;

use share::read_all_stream;

fn read_json_disk(c: &mut Criterion) {
    let flate_file = tempfile::NamedTempFile::new().unwrap();
    let gzip_file = tempfile::NamedTempFile::new().unwrap();
    let bzip_file = tempfile::NamedTempFile::new().unwrap();
    let xz_file = tempfile::NamedTempFile::new().unwrap();

    println!("flate_file path: {}", flate_file.path().display());
    {
        // get flate file
        let mut wfile = flate_file.reopen().unwrap();
        let resp =
            reqwest::blocking::get("https://conda.anaconda.org/conda-forge/linux-64/repodata.json")
                .unwrap()
                .bytes()
                .unwrap();
        let mut reader = resp.as_ref();

        let mut content = Vec::new();
        reader.read_to_end(&mut content).unwrap();
        wfile.write_all(&content[0..2usize.pow(20)]).unwrap();

        wfile.flush().unwrap();

        // gzip compression
        let mut gzip_writer = niffler::get_writer(
            Box::new(gzip_file.reopen().unwrap()),
            niffler::compression::Format::Gzip,
            niffler::level::Level::Five,
        )
        .unwrap();
        gzip_writer.write_all(&content[0..2usize.pow(20)]).unwrap();
        gzip_writer.flush().unwrap();

        // bzip compression
        let mut bzip_writer = niffler::get_writer(
            Box::new(bzip_file.reopen().unwrap()),
            niffler::compression::Format::Bzip,
            niffler::level::Level::Five,
        )
        .unwrap();
        bzip_writer.write_all(&content[0..2usize.pow(20)]).unwrap();
        bzip_writer.flush().unwrap();

        // xz compression
        let mut xz_writer = niffler::get_writer(
            Box::new(xz_file.reopen().unwrap()),
            niffler::compression::Format::Lzma,
            niffler::level::Level::Five,
        )
        .unwrap();
        xz_writer.write_all(&content[0..2usize.pow(20)]).unwrap();
        xz_writer.flush().unwrap();
    }

    let mut g = c.benchmark_group("json");

    g.bench_function("flate", |b| {
        b.iter(|| {
            read_all_stream(
                niffler::get_reader(Box::new(std::fs::File::open(flate_file.path()).unwrap()))
                    .unwrap()
                    .0,
            );
        })
    });

    g.bench_function("flate_buffered", |b| {
        b.iter(|| {
            read_all_stream(niffler::from_path(flate_file.path()).unwrap().0);
        })
    });

    g.bench_function("gzip", |b| {
        b.iter(|| {
            read_all_stream(
                niffler::get_reader(Box::new(std::fs::File::open(gzip_file.path()).unwrap()))
                    .unwrap()
                    .0,
            );
        })
    });

    g.bench_function("gzip_buffered", |b| {
        b.iter(|| {
            read_all_stream(niffler::from_path(gzip_file.path()).unwrap().0);
        })
    });

    g.bench_function("bzip", |b| {
        b.iter(|| {
            read_all_stream(
                niffler::get_reader(Box::new(std::fs::File::open(bzip_file.path()).unwrap()))
                    .unwrap()
                    .0,
            );
        })
    });

    g.bench_function("bzip_buffered", |b| {
        b.iter(|| {
            read_all_stream(niffler::from_path(bzip_file.path()).unwrap().0);
        })
    });

    g.bench_function("xz", |b| {
        b.iter(|| {
            read_all_stream(
                niffler::get_reader(Box::new(std::fs::File::open(xz_file.path()).unwrap()))
                    .unwrap()
                    .0,
            );
        })
    });

    g.bench_function("xz_buffered", |b| {
        b.iter(|| {
            read_all_stream(niffler::from_path(xz_file.path()).unwrap().0);
        })
    });
}

fn setup(c: &mut Criterion) {
    read_json_disk(c);
}

criterion_group!(benches, setup);
criterion_main!(benches);
