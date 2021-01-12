//! Command to run benches:
//! `cargo +nightly bench`
//! bench is nightly feature.
//! So use `+nightly` to tempolary(not change default) change toolchain for nightly

#![feature(test)]

use ply::PLYFile;

extern crate test;
use test::Bencher;

const PATH: &str = "./benches/ply_files/your_file_name.ply";

/// Measure times to read file which can found with PATH  
/// In my enviroment, 564,867,610 ns (0.5sec) is measured to read ply file (size: about 19MB, 850000 vertex)
#[bench]
fn read_time(b: &mut Bencher) {
    b.iter(|| {
        PLYFile::from_file(PATH).unwrap();
    });
}
