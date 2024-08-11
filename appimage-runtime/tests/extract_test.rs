use std::path::PathBuf;

use appimage_runtime::extract;

#[test]
fn list_files_test() {
    let sqfs = PathBuf::from("tests/test.sqfs");

    let files = extract::list_files(&sqfs, 0).unwrap();

    assert_eq!(
        files,
        vec![
            PathBuf::from("/"),
            PathBuf::from("/AppRun"),
            PathBuf::from("/other"),
            PathBuf::from("/other/path"),
            PathBuf::from("/other/path/file.txt"),
        ]
    );
}
