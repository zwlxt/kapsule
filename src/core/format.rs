use crate::{util::{self, encoding::DecoderExt}, ArchiveFile, Entry, ExtractDest};
use anyhow::{bail, Result, anyhow};
use flate2::read::GzDecoder;
use std::{fs::File, path::Path};
use tar::{Archive, EntryType};

use zip::ZipArchive;

pub struct TgzFile {
    archive: Archive<GzDecoder<File>>,
}

impl TgzFile {
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;
        let tar_file = GzDecoder::new(file);
        let archive = Archive::new(tar_file);
        Ok(Self { archive })
    }
}

impl ArchiveFile for TgzFile {
    fn entries<'a>(&'a mut self) -> Result<Box<dyn Iterator<Item = Result<Entry>> + 'a>> {
        let iter = self
            .archive
            .entries()?
            .map(|entry| Entry::try_from(&entry?));
        Ok(Box::new(iter))
    }

    fn extract(&mut self, entry: &str, dest: ExtractDest) -> Result<()> {
        let found = self.archive.entries()?.find(|ent| {
            if let Ok(ent) = ent {
                ent.path().map_or(false, |p| p.to_string_lossy() == entry)
            } else {
                false
            }
        });

        match found {
            Some(Ok(mut entry)) => {
                match dest {
                    ExtractDest::File(f) => {
                        match entry.header().entry_type() {
                            EntryType::Directory => {
                                todo!()
                            },
                            _ => {
                                // create file for extraction
                                let mut dest_file = File::create(f)?;
                                std::io::copy(&mut entry, &mut dest_file)?;
                            }
                        }
                    }
                    ExtractDest::Dir(d) => {
                        entry.unpack_in(d)?;
                    }
                };
            }
            Some(Err(e)) => bail!(e),
            None => bail!("entry not found"),
        }

        Ok(())
    }

    fn extract_all<P: AsRef<Path>>(&mut self, dest: P) -> Result<()> {
        Ok(self.archive.unpack(dest)?)
    }
}

impl<R: std::io::Read> TryFrom<&tar::Entry<'_, R>> for Entry {
    type Error = anyhow::Error;

    fn try_from(entry: &tar::Entry<'_, R>) -> Result<Self, Self::Error> {
        let path_raw = entry.path_bytes();

        Ok(Self {
            name: path_raw.guess_and_decode(),
        })
    } 
}

pub struct ZipFile {
    archive: ZipArchive<File>,
}

impl ZipFile {
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;
        let zip_file = ZipArchive::new(file)?;
        Ok(Self { archive: zip_file })
    }
}

impl ArchiveFile for ZipFile {
    fn entries<'a>(&'a mut self) -> Result<Box<dyn Iterator<Item = Result<Entry>> + 'a>> {
        Ok(Box::new(ZipEntries {
            zip_file: self,
            idx: 0,
        }))
    }

    fn extract(&mut self, entry: &str, dest: ExtractDest) -> Result<()> {
        match dest {
            ExtractDest::File(f) => {
                let mut file = self.archive.by_name(entry)?;
                if file.is_dir() {
                    todo!()
                } else {
                    let mut dest_file = File::create(f)?;
                    std::io::copy(&mut file, &mut dest_file)?;
                }
            }
            ExtractDest::Dir(_) => todo!(),
        }
        Ok(())
    }

    fn extract_all<P: AsRef<Path>>(&mut self, dest: P) -> Result<()> {
        Ok(self.archive.extract(dest)?)
    }
}

pub struct ZipEntries<'a> {
    zip_file: &'a mut ZipFile,
    idx: usize,
}

impl Iterator for ZipEntries<'_> {
    type Item = Result<Entry>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.zip_file.archive.len() {
            let r = match self.zip_file.archive.by_index(self.idx) {
                Ok(entry) => {
                    self.idx += 1;
            
                    Ok(Entry {
                        name: entry.name_raw().guess_and_decode(),
                    })
                },
                Err(e) => Err(anyhow!(e)),
            };
            Some(r)
        } else { 
            None
        }
    }
}
