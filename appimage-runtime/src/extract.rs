use anyhow::Result;
use backhand;
use std::fs::{File, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use glob;

fn make_filesystem_reader(
    appimage: &PathBuf,
    fs_offset: u64,
) -> Result<backhand::FilesystemReader> {
    let reader = File::open(appimage)?;
    let reader = std::io::BufReader::new(reader);
    let squashfs = backhand::Squashfs::from_reader_with_offset(reader, fs_offset)?;
    let filesystem = squashfs.into_filesystem_reader()?;
    Ok(filesystem)
}

pub fn list_files(appimage: &PathBuf, fs_offset: u64) -> Result<Vec<PathBuf>> {
    let filesystem = make_filesystem_reader(appimage, fs_offset)?;
    let nodes = filesystem.root.nodes;
    let files = nodes.iter().map(|node| node.fullpath.clone()).collect();
    Ok(files)
}

pub fn extract_files(
    appimage: &PathBuf,
    fs_offset: u64,
    pattern: Option<&glob::Pattern>,
    target: &Path,
) -> Result<PathBuf> {
    let filesystem = make_filesystem_reader(appimage, fs_offset)?;
    for node in &filesystem.root.nodes {
        if let Some(p) = pattern {
            if !p.matches(node.fullpath.to_str().unwrap()) {
                continue;
            }
        }
        let path = target.join(node.fullpath.strip_prefix("/")?);
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent)?;
        match &node.inner {
            backhand::InnerNode::File(file_reader) => {
                let mut file_out = File::create(&path)?;
                let file_handler = &filesystem.file(&file_reader.basic);
                let mut reader = file_handler.reader();
                std::io::copy(&mut reader, &mut file_out)?;
                println!("Extracted: {:?}", path);
            }
            backhand::InnerNode::Dir(_dir) => {
                std::fs::create_dir_all(&path)?;
                println!("Created directory: {:?}", path);
            }
            backhand::InnerNode::Symlink(backhand::SquashfsSymlink { link }) => {
                std::os::unix::fs::symlink(link, &path)?;
                println!("Created symlink: {:?} -> {:?}", path, link);
            }
            _ => {
                // TODO: character device, block device, pipe, socket
                println!("Unsupported node type: {:?}", node.inner);
            }
        }
        let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(node.header.mtime.into());
        let file = File::open(path)?;
        file.set_modified(system_time)?;
        let perm = Permissions::from_mode(node.header.permissions.into());
        file.set_permissions(perm)?;
    }
    Ok(target.to_path_buf())
}
