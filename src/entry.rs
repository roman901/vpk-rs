use binread::BinRead;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct VPKEntry {
    pub dir_entry: VPKDirectoryEntry,
    pub archive_path: String,
    pub preload_data: Vec<u8>,
}

impl VPKEntry {
    pub fn get(&self) -> Result<Cow<[u8]>, Error> {
        let mut reader = self.reader()?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(Cow::from(buf))
    }

    /// Create a [`VPKEntryReader`].
    pub fn reader(&self) -> Result<VPKEntryReader<'_>, Error> {
        let file = if self.dir_entry.archive_index == 0x7fff {
            None
        } else {
            let mut file = File::open(&self.archive_path)?;
            file.seek(SeekFrom::Start(self.dir_entry.archive_offset as u64))?;
            Some(file.take(self.dir_entry.file_length as u64))
        };

        Ok(VPKEntryReader::new(&self.preload_data, file))
    }
}

/// A reader over the [`VPKEntry`].
pub enum VPKEntryReader<'a> {
    /// Only preloaded data must be read.
    PreloadedOnly {
        preloaded_data: std::io::Cursor<&'a [u8]>,
    },
    /// Read from preloaded data first and then the file.
    PreloadAndFile {
        /// Length of the preloaded data.
        preloaded_data_len: usize,
        /// Number of bytes of the preloaded data read so far.
        preloaded_bytes_read: usize,
        /// Preloaded data.
        preloaded_data: std::io::Cursor<&'a [u8]>,
        /// The file that must be read.
        file: std::io::Take<File>,
    },
    /// Only the file must be read.
    FileOnly { file: std::io::Take<File> },
}

impl Read for VPKEntryReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            VPKEntryReader::PreloadedOnly { preloaded_data } => preloaded_data.read(buf),
            VPKEntryReader::PreloadAndFile {
                preloaded_data_len,
                preloaded_bytes_read,
                preloaded_data,
                file,
            } => {
                if preloaded_bytes_read >= preloaded_data_len {
                    file.read(buf)
                } else {
                    let bytes_read = preloaded_data.read(buf)?;

                    let bytes_read = if bytes_read < buf.len() {
                        let file_bytes_read = file.read(&mut buf[bytes_read..])?;
                        bytes_read + file_bytes_read
                    } else {
                        bytes_read
                    };

                    *preloaded_bytes_read += bytes_read;

                    Ok(bytes_read)
                }
            }
            VPKEntryReader::FileOnly { file } => file.read(buf),
        }
    }
}

impl<'a> VPKEntryReader<'a> {
    /// Create a new [`VPKEntryReader`].
    pub fn new(preloaded_data: &'a [u8], file: Option<std::io::Take<File>>) -> Self {
        match file {
            Some(file) => {
                if preloaded_data.is_empty() {
                    Self::FileOnly { file }
                } else {
                    Self::PreloadAndFile {
                        preloaded_data_len: preloaded_data.len(),
                        preloaded_bytes_read: 0,
                        preloaded_data: std::io::Cursor::new(preloaded_data),
                        file,
                    }
                }
            }
            None => Self::PreloadedOnly {
                preloaded_data: std::io::Cursor::new(preloaded_data),
            },
        }
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
