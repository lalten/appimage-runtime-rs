use appimage_mount::mount;
use appimage_runtime::extract;
use appimage_runtime::util;
use std::path::PathBuf;
use std::thread;

#[derive(PartialEq)]
enum Action {
    Extract,
    ExtractAndRun,
    Help,
    List,
    Mount,
    MountAndRun,
    Offset,
}

fn main() {
    let orig_args = Vec::from_iter(std::env::args_os());
    let cwd = match std::env::var("BUILD_WORKING_DIRECTORY") {
        Ok(bwd) => PathBuf::from(bwd),
        _ => std::env::current_dir().unwrap(),
    };
    let argv0 = orig_args.first().unwrap().clone().into_string().unwrap();
    let (maybe_arg1, args) = util::consume_appimage_arg(&orig_args[1..]);
    let action = match maybe_arg1 {
        None => Action::MountAndRun,
        Some(arg1) => match arg1.to_string_lossy().into_owned().as_str() {
            "--appimage-extract" => Action::Extract,
            "--appimage-extract-and-run" => Action::ExtractAndRun,
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

    match action {
        Action::Offset => {
            println!("{fs_offset}");
            return;
        }
        Action::List => {
            for file in extract::list_files(appimage, fs_offset).unwrap() {
                println!("{}", file.to_string_lossy());
            }
            return;
        }
        _ => (),
    };

    let pattern = match action {
        Action::Extract => match args.first() {
            Some(arg) => {
                let pattern = arg.to_string_lossy().into_owned();
                Some(glob::Pattern::new(pattern.as_str()).unwrap())
            }
            None => None,
        },
        _ => None,
    };

    let appdir = match action {
        Action::Mount | Action::MountAndRun => {
            let tmpdir = util::make_hashed_tempdirname(appimage, ".mount_").unwrap();
            mount::squashfuse_mount(appimage, fs_offset, &tmpdir)
                .unwrap_or_else(|err| {
                    panic!("Failed to mount {appimage:?} at offset {fs_offset}: {err}")
                })
                .to_owned()
        }
        Action::ExtractAndRun => {
            let tmpdir = util::make_hashed_tempdirname(appimage, "appimage_extracted_").unwrap();
            util::remove_when_done(&tmpdir).unwrap();
            extract::extract_files(appimage, fs_offset, pattern.as_ref(), &tmpdir)
                .unwrap()
                .to_owned()
        }
        Action::Extract => {
            let tmpdir = cwd.join("squashfs-root");
            extract::extract_files(appimage, fs_offset, pattern.as_ref(), &tmpdir)
                .unwrap()
                .to_owned()
        }
        _ => unreachable!(),
    };

    match action {
        Action::Mount => {
            println!("{}", appdir.to_string_lossy());
            loop {
                thread::park();
            }
        }
        Action::Extract => return,
        _ => (),
    }

    std::env::set_var("APPDIR", &appdir);
    std::env::set_var("APPIMAGE", appimage);
    std::env::set_var("ARGV0", argv0);
    std::env::set_var("OWD", cwd);

    let entrypoint = &appdir.join("AppRun");
    let err = exec::Command::new(entrypoint).args(args).exec();
    panic!("Failed to execute {entrypoint:?}: {err}")
}
