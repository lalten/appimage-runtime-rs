use appimage_mount::mount;

fn create_squashfs(path: &std::path::PathBuf, offset: u64) {
    use backhand::*;
    let file = std::fs::File::create(path).unwrap();
    file.set_len(offset).unwrap();
    let mut fs = FilesystemWriter::default();
    fs.set_compressor(FilesystemCompressor::new(compression::Compressor::Gzip, None).unwrap());
    fs.push_dir("mydir", NodeHeader::default()).unwrap();
    fs.push_file(
        std::io::Cursor::new(b"Hello World!"),
        "mydir/myfile",
        NodeHeader::default(),
    )
    .unwrap();
    fs.write_with_offset(file, offset).expect("write squashfs");
}

#[test]
fn squashfuse_mount_works() {
    let squashfs = tempfile::NamedTempFile::new().unwrap();
    let path = std::path::PathBuf::from(&squashfs.path());
    let offset = 1234;
    create_squashfs(&path, offset);

    let mountpoint = mount::squashfuse_mount(&path, offset).unwrap();

    let content = std::fs::read_to_string(&mountpoint.join("mydir/myfile")).unwrap();
    assert_eq!(content, "Hello World!");
}

#[test]
fn squashfuse_mount_fail() {
    let path = std::path::PathBuf::from("/dev/null");
    let err = mount::squashfuse_mount(&path, 0).unwrap_err();
    assert!(err.to_string().contains("This doesn't look like a squashfs image."), "unexpected err: {}", err);
}
