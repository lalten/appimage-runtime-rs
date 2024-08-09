use appimage_mount::mount;
use appimage_runtime::util;

#[derive(PartialEq)]
enum Action {
    Help,
    Mount,
    MountAndRun,
    Offset,
}

fn main() {
    let (maybe_arg1, args) = util::consume_appimage_arg(&std::env::args_os().skip(1).collect());
    let action = match maybe_arg1 {
        None => Action::MountAndRun,
        Some(arg1) => match arg1.to_string_lossy().into_owned().as_str() {
            "--appimage-help" => Action::Help,
            "--appimage-mount" => Action::Mount,
            "--appimage-offset" => Action::Offset,
            arg => panic!("Invalid --appimage- arg: {arg:?}. Try --appimage-help."),
        },
    };

    let argv0 = std::env::args_os()
        .next()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    if action == Action::Help {
        util::print_help(&argv0);
        return;
    }

    let prog = util::get_appimage_path();
    let appimage = &prog
        .read_link()
        .unwrap_or_else(|err| panic!("readlink failed on {prog:?}: {err}"));
    let fs_offset = util::get_elf_size(&appimage);
    if action == Action::Offset {
        println!("{fs_offset}");
        return;
    }

    let appdir = mount::squashfuse_mount(&appimage, fs_offset)
        .unwrap_or_else(|err| panic!("Failed to mount {appimage:?} at offset {fs_offset}: {err}"));
    if action == Action::Mount {
        println!("{}", appdir.to_string_lossy());
        return;
    }

    std::env::set_var("APPDIR", &appdir);
    std::env::set_var("APPIMAGE", &appimage);
    std::env::set_var("ARGV0", &argv0);
    std::env::set_var("OWD", std::env::current_dir().unwrap());

    let entrypoint = &appdir.join("AppRun");
    let err = exec::Command::new(entrypoint).args(&args).exec();
    panic!("Failed to execute {entrypoint:?}: {err}")
}
