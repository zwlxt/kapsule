pub mod format;
pub mod util;

use std::{path::Path};
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
}

pub trait ArchiveFile {
    fn entries<'a>(&'a mut self) -> Result<Box<dyn Iterator<Item = Result<Entry>> + 'a>>;
    fn extract(&mut self, entry: &str, dest: ExtractDest) -> Result<()>;
    fn extract_all<P: AsRef<Path>>(&mut self, dest: P) -> Result<()>;
}

pub enum ExtractDest<'a> {
    File(&'a dyn AsRef<Path>),
    Dir(&'a dyn AsRef<Path>),
}

pub type ProgressCallback = Box<dyn Fn(usize, usize)>;

pub struct ProgressMonitor {
    pub progress: ProgressCallback,
    pub overall_progress: ProgressCallback,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use zip::ZipArchive;

    use crate::{format::{TgzFile, ZipFile}, ArchiveFile, ExtractDest};

    #[test]
    fn full() {
        let file =
            File::open("testcases/a.zip")
                .unwrap();
        let mut zip_file = ZipArchive::new(file).unwrap();
        let mut a = zip_file.by_index(3).unwrap();
        println!("{}", &a.enclosed_name().unwrap().to_string_lossy());
        // let mut out_file = File::create("out").unwrap();
        std::io::copy(&mut a, &mut std::io::stdout()).unwrap();
    }

    #[test]
    fn extract_tgz_single_to_dir() {
        let mut file = TgzFile::open("testcases/a.tar.gz").unwrap();
        file.extract(
            "druid-0.7.0/druid/examples/hello.rs",
            ExtractDest::Dir(&"."),
        )
        .unwrap();
    }

    #[test]
    fn extract_tgz_single_to_file() {
        let mut file = TgzFile::open("testcases/a.tar.gz").unwrap();
        file.extract(
            "druid-0.7.0/druid/examples/hello.rs",
            ExtractDest::File(&"./wtf.rs"),
        )
        .unwrap();
    }

    #[test]
    fn list_tgz_files() {
        let mut file = TgzFile::open("testcases/a.tar.gz").unwrap();
        for entry in file.entries().unwrap() {
            println!("{}", entry.unwrap().name);
        }
    }

    #[test]
    fn list_zip_files() {
        let mut file = ZipFile::open(
            "testcases/gbk.zip",
        )
        .unwrap();
        for entry in file.entries().unwrap() {
            println!("{}", entry.unwrap().name);
        }
    }
}
