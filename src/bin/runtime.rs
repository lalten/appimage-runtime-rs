use appimage_mount::mount;
use appimage_runtime::util;

fn main() {
    let prog = util::get_appimage_path();
    let appimage = &prog
        .read_link()
        .unwrap_or_else(|err| panic!("readlink failed on {prog:?}: {err}"));
    let fs_offset = util::get_elf_size(&appimage);

    let appdir = mount::squashfuse_mount(&appimage, fs_offset).unwrap();

    std::env::set_var("APPDIR", &appdir);
    std::env::set_var("APPIMAGE", &appimage);
    std::env::set_var("ARGV0", std::env::args_os().next().unwrap());
    std::env::set_var("OWD", std::env::current_dir().unwrap());

    let entrypoint = &appdir.join("AppRun");
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let err = exec::Command::new(entrypoint).args(&argv).exec();
    panic!("Failed to execute {entrypoint:?}: {err}")
}
