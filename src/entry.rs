use std::borrow::Cow;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom};
use binrw::BinRead;

#[derive(Debug)]
pub struct VPKEntry {
    pub dir_entry: VPKDirectoryEntry,
    pub archive_path: String,
    pub preload_data: Vec<u8>,
}

impl VPKEntry {
    pub fn get(&self) -> Result<Cow<[u8]>, Error> {
        if self.dir_entry.archive_index == 0x7fff {
            // Return internal preload_data
            return Ok(Cow::Borrowed(&self.preload_data));
        }

        let mut buf = vec![0; self.dir_entry.file_length as usize];
        let mut file = File::open(&self.archive_path)?;
        file.seek(SeekFrom::Start(self.dir_entry.archive_offset as u64))?;
        file.take(self.dir_entry.file_length as u64).read(&mut buf)?;
        Ok(Cow::Owned(buf))
    }
}

impl Read for VPKEntry {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.dir_entry.archive_index == 0x7fff {
            // Return internal preload_data
            buf.copy_from_slice(&self.preload_data);
            return Ok(buf.len());
        }

        let mut file = File::open(&self.archive_path)?;
        file.seek(SeekFrom::Start(self.dir_entry.archive_offset as u64))?;
        file.take(self.dir_entry.file_length as u64).read(buf)?;

        Ok(self.dir_entry.file_length as usize)
    }
}

#[derive(Debug, BinRead)]
pub struct VPKDirectoryEntry {
    pub crc32: u32,
    pub preload_length: u16,
    pub archive_index: u16,
    pub archive_offset: u32,
    pub file_length: u32,
    pub suffix: u16,
}