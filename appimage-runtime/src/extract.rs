use anyhow::Result;
use backhand;
use std::path::PathBuf;

pub fn list_files(appimage: &PathBuf, fs_offset: u64) -> Result<Vec<PathBuf>> {
    let reader = std::fs::File::open(appimage)?;
    let reader = std::io::BufReader::new(reader);
    let squashfs = backhand::Squashfs::from_reader_with_offset(reader, fs_offset)?;
    let filesystem = squashfs.into_filesystem_reader()?;
    let nodes = filesystem.root.nodes;
    let files = nodes.iter().map(|node| node.fullpath.clone()).collect();
    Ok(files)
}
