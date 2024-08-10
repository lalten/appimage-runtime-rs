use appimage_mount::mount;
use appimage_runtime::extract;
use appimage_runtime::util;

#[derive(PartialEq)]
enum Action {
    Help,
    List,
    Mount,
    MountAndRun,
    Offset,
}

fn main() {
    let orig_args = Vec::from_iter(std::env::args_os());
    let argv0 = orig_args.first().unwrap().clone().into_string().unwrap();
    let (maybe_arg1, args) = util::consume_appimage_arg(&orig_args[1..]);
    let action = match maybe_arg1 {
        None => Action::MountAndRun,
        Some(arg1) => match arg1.to_string_lossy().into_owned().as_str() {
            "--appimage-help" => Action::Help,
            "--appimage-mount" => Action::Mount,
            "--appimage-list" => Action::List,
            "--appimage-offset" => Action::Offset,
            arg => panic!("Invalid --appimage- arg: {arg:?}. Try --appimage-help."),
        },
    };

    if action == Action::Help {
        util::print_help(&argv0);
        return;
    }

    let prog = util::get_appimage_path();
    let appimage = &prog
        .read_link()
        .unwrap_or_else(|err| panic!("readlink failed on {prog:?}: {err}"));
    let fs_offset = util::get_elf_size(appimage).unwrap();
    if action == Action::Offset {
        println!("{fs_offset}");
        return;
    }

    if action == Action::List {
        for file in extract::list_files(appimage, fs_offset).unwrap() {
            println!("{}", file.to_string_lossy());
        }
        return;
    }

    let appdir = mount::squashfuse_mount(appimage, fs_offset)
        .unwrap_or_else(|err| panic!("Failed to mount {appimage:?} at offset {fs_offset}: {err}"));
    if action == Action::Mount {
        println!("{}", appdir.to_string_lossy());
        return;
    }

    std::env::set_var("APPDIR", &appdir);
    std::env::set_var("APPIMAGE", appimage);
    std::env::set_var("ARGV0", argv0);
    std::env::set_var("OWD", std::env::current_dir().unwrap());

    let entrypoint = &appdir.join("AppRun");
    let err = exec::Command::new(entrypoint).args(args).exec();
    panic!("Failed to execute {entrypoint:?}: {err}")
}
