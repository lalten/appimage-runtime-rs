use appimage_runtime::util;

use runfiles::Runfiles;

#[test]
fn squashfuse_main_elf_size_is_binary_size() {
    let r = Runfiles::create().unwrap();
    let binary = runfiles::rlocation!(r, "squashfuse/ll_main");
    let file_size = binary.metadata().unwrap().len();

    let elf_size = util::get_elf_size(&binary);

    assert_eq!(file_size, elf_size);
}
