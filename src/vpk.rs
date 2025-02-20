use crate::entry::*;
use crate::structs::*;
use crate::Error;

use binread::BinReaderExt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

const VPK_SIGNATURE: u32 = 0x55aa1234;
const VPK_SELF_HASHES_LENGTH: u32 = 48;

/// Valve pack (VPK).
///
/// Package format used by post-GCF Source engine games to store game
/// content.
///
/// There is usually a directory VPK which can refers to other VPKs at
/// the same directory level via indices as variations of the loaded
/// VPKs file name. The directory VPK is usually named
/// `pak01_dir.vpk`.
#[derive(Debug)]
pub struct VPK {
    /// Header length.
    pub header_length: u32,
    /// [`VPKHeader`].
    pub header: VPKHeader,
    /// [`VPKHeaderV2`].
    pub header_v2: Option<VPKHeaderV2>,
    /// [`VPKHeaderV2Checksum`].
    pub header_v2_checksum: Option<VPKHeaderV2Checksum>,

    /// Tree of the VPK containing all the [`VPKEntry`]s.
    pub tree: HashMap<String, VPKEntry>,

    /// Path to root VPK.
    ///
    /// This is the path to the `.vpk` that [`VPK`] has read.
    pub root_path: Arc<PathBuf>,
}

impl VPK {
    /// Read the [`VPK`] at the given [`Path`].
    #[doc(alias = "open")]
    pub fn read(dir_path: impl AsRef<Path>) -> Result<VPK, Error> {
        let dir_path = dir_path.as_ref().to_path_buf();
        let file = File::open(&dir_path)?;

        let mut reader = BufReader::new(file);

        // Read main VPK header
        let header: VPKHeader = reader.read_le()?;

        if header.signature != VPK_SIGNATURE {
            return Err(Error::InvalidSignature);
        }
        if header.version > 2 {
            return Err(Error::UnsupportedVersion(header.version));
        }

        let mut vpk = VPK {
            header_length: 4 * 3,
            header,
            header_v2: None,
            header_v2_checksum: None,
            tree: HashMap::new(),
            root_path: Arc::new(dir_path),
        };

        if vpk.header.version == 2 {
            let header_v2: VPKHeaderV2 = reader.read_le()?;

            if header_v2.self_hashes_length != VPK_SELF_HASHES_LENGTH {
                return Err(Error::HashSizeMismatch);
            }
            vpk.header_length += 4 * 4;

            let checksum_offset: u32 = vpk.header.tree_length
                + header_v2.embed_chunk_length
                + header_v2.chunk_hashes_length;
            reader.seek(SeekFrom::Current(checksum_offset as i64))?;

            let header_v2_checksum: VPKHeaderV2Checksum = reader.read_le()?;

            vpk.header_v2 = Some(header_v2);
            vpk.header_v2_checksum = Some(header_v2_checksum);

            // Return seek to initial position - after header
            let header_length = mem::size_of::<VPKHeader>() + mem::size_of::<VPKHeaderV2>();
            reader.seek(SeekFrom::Start(header_length as u64))?;
        }

        let vpk_root_parent = vpk.root_path.parent().expect("file always in a directory");
        let vpk_root_file_name = vpk
            .root_path
            .file_name()
            .ok_or(Error::FilenameNotAvailable)?
            .to_str()
            .ok_or(Error::FilenameNotRepresentableAsStr)?;
        let mut vpk_paths = HashMap::new();
        vpk_paths.insert(0x7fff, vpk.root_path.clone());

        let mut vpk_path_for_archive_index =
            |archive_index: u16| {
                vpk_paths
                    .entry(archive_index)
                    .or_insert_with(|| {
                        Arc::new(vpk_root_parent.join(
                            vpk_root_file_name.replace("dir", &format!("{:03}", archive_index)),
                        ))
                    })
                    .clone()
            };

        // Read index tree
        loop {
            let ext = read_cstring(&mut reader)?;
            if ext.is_empty() {
                break;
            }

            loop {
                let mut path = read_cstring(&mut reader)?;
                if path.is_empty() {
                    break;
                }
                if path != " " {
                    path += "/";
                } else {
                    path.clear();
                }

                loop {
                    let name = read_cstring(&mut reader)?;
                    if name.is_empty() {
                        break;
                    }

                    let mut dir_entry: VPKDirectoryEntry = reader.read_le()?;

                    if dir_entry.suffix != 0xffff {
                        return Err(Error::MalformedIndex);
                    }

                    if dir_entry.archive_index == 0x7fff {
                        dir_entry.archive_offset += vpk.header_length + vpk.header.tree_length;
                    }

                    let preload_length = dir_entry.preload_length;
                    let archive_path = (dir_entry.file_length != 0)
                        .then(|| vpk_path_for_archive_index(dir_entry.archive_index));
                    let mut vpk_entry = VPKEntry {
                        dir_entry,
                        archive_path,
                        preload_data: vec![0u8; preload_length as usize],
                    };

                    reader
                        .by_ref()
                        .take(vpk_entry.dir_entry.preload_length as u64)
                        .read_exact(&mut vpk_entry.preload_data)?;

                    vpk.tree
                        .insert(format!("{}{}.{}", path, name, ext), vpk_entry);
                }
            }
        }

        Ok(vpk)
    }
}

fn read_cstring(reader: &mut BufReader<File>) -> Result<String, Error> {
    let mut string: String = String::new();

    let mut buf = [0u8];
    loop {
        reader.by_ref().read_exact(&mut buf)?;
        //println!("{:?}", buf[0]);
        if buf[0] == 0 {
            break;
        } else {
            string.push(buf[0] as char);
        }
    }

    Ok(string)
}
