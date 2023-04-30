use std::fs::{File, DirEntry, self};
use std::io;
use std::path::Path;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;

fn main() -> Result<(), std::io::Error> {
    let tar_gz: File = File::create("archive.tar.gz")?;
    let enc: GzEncoder<File> = GzEncoder::new(tar_gz, Compression::default());
    let mut tar: Builder<GzEncoder<File>> = Builder::new(enc);
    Ok(())
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}