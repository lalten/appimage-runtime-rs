use anyhow::{Context, Result};
use base62;
use std::{ffi::OsString, hash::Hasher, io::Read, path::PathBuf};

pub fn get_appimage_path() -> PathBuf {
    let path = std::env::var("TARGET_APPIMAGE").unwrap_or("/proc/self/exe".to_string());
    let prog = PathBuf::from(path);
    match prog.is_relative() {
        true => {
            let bwd = std::env::var("BUILD_WORKING_DIRECTORY").unwrap_or(".".to_string());
            PathBuf::from(bwd).join(prog)
        }
        false => prog,
    }
}

pub fn get_elf_size(path: &PathBuf) -> Result<u64> {
    use elf::endian::AnyEndian;
    use elf::ElfStream;
    let io = std::fs::File::open(path).with_context(|| format!("Opening elf {path:?}"))?;
    let elf = ElfStream::<AnyEndian, _>::open_stream(io).context("Parsing elf")?;
    let sht_end = elf.ehdr.e_shoff + elf.ehdr.e_shentsize as u64 * elf.ehdr.e_shnum as u64;
    let last_section_end = match elf.segments().last() {
        Some(section) => section.p_offset + section.p_filesz,
        None => 0,
    };
    Ok(std::cmp::max(sht_end, last_section_end))
}

pub fn print_help(argv0: &String) {
    println!("\
appimage-runtime-rs: A type-2 AppImage runtime implementation in Rust, built with Bazel.

Usage: {argv0} [OPTION] [ARG]...

Options listed here will be consumed by the runtime. All other options will be passed to the application.

--appimage-extract-and-run      Extract content from embedded filesystem image and execute the AppRun
--appimage-extract [<pattern>]  Extract content from embedded filesystem image. If pattern is passed, only extract matching files
--appimage-help                 Print this help
--appimage-list                 List content from embedded filesystem image
--appimage-mount                Mount embedded filesystem image and print mount point. Stop with Ctrl-C.
--appimage-offset               Print byte offset to start of embedded filesystem image

It is an error to pass more than one of the above options.
It is an error to pass a --appimage-* option not listed above as the first argument.
");
}

pub fn consume_appimage_arg(args: &[OsString]) -> (Option<OsString>, &[OsString]) {
    if let Some(arg1) = args.first() {
        if arg1.to_string_lossy().starts_with("--appimage-") {
            return (Some(arg1.clone()), &args[1..]);
        }
    }
    (None, args)
}

pub fn get_hash(path: &PathBuf) -> Result<String> {
    const BUFFER_LEN: usize = 1024 * 1024;
    let mut buffer = [0u8; 1024 * 1024];

    let mut reader = std::fs::File::open(path)?;
    let mut hasher = seahash::SeaHasher::new();

    loop {
        let read_count = reader.read(&mut buffer)?;
        hasher.write(&buffer[..read_count]);
        if read_count != BUFFER_LEN {
            break;
        }
    }

    Ok(base62::encode(hasher.finish()))
}

pub fn make_hashed_tempdirname<S: Into<String>>(appimage: &PathBuf, prefix: S) -> Result<PathBuf> {
    let hash = get_hash(appimage)?;
    let tempdir = std::env::temp_dir();
    Ok(tempdir.join(prefix.into() + &hash))
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn get_elf_size_devnull() {
        let devnull = std::path::PathBuf::from("/dev/null");
        let err = get_elf_size(&devnull).unwrap_err();
        assert_eq!(format!("{err:#}"), "Parsing elf: Bad offset: 0x10");
    }

    #[test]
    fn get_elf_size_nosuchfile() {
        let devnull = std::path::PathBuf::from("/invalid");
        let err = get_elf_size(&devnull).unwrap_err();
        assert_eq!(
            format!("{err:#}"),
            "Opening elf \"/invalid\": No such file or directory (os error 2)"
        );
    }

    #[test]
    fn get_elf_size_self() {
        let devnull = std::path::PathBuf::from("/proc/self/exe");
        let size = get_elf_size(&devnull).unwrap();
        assert!(size > 0);
    }

    #[test]
    fn consume_appimage_arg_empty() {
        let args_in = Vec::new();

        let (arg1, args_out) = consume_appimage_arg(&args_in);

        assert!(arg1.is_none());
        assert_eq!(&args_out, &args_in);
    }

    #[test]
    fn consume_appimage_arg_consume() {
        let args_in = vec![
            OsString::from("--appimage-test-consume-args"),
            OsString::from("Hello"),
            OsString::from("World"),
        ];

        let (arg1, args_out) = consume_appimage_arg(&args_in);

        assert_eq!(arg1.unwrap(), args_in[0]);
        assert_eq!(args_out, &args_in[1..]);
    }

    #[test]
    fn consume_appimage_arg_passthrough() {
        let args_in = vec![OsString::from("Hello"), OsString::from("World")];

        let (arg1, args_out) = consume_appimage_arg(&args_in);

        assert!(arg1.is_none());
        assert_eq!(&args_out, &args_in.as_slice());
    }

    #[test]
    fn consume_appimage_arg_passthrough_when_not_first() {
        let args_in = vec![
            OsString::from("Hello"),
            OsString::from("--appimage-test-consume-args"),
            OsString::from("World"),
        ];

        let (arg1, args_out) = consume_appimage_arg(&args_in);

        assert!(arg1.is_none());
        assert_eq!(&args_out, &args_in.as_slice());
    }

    #[test]
    fn get_hash_devnull() {
        let devnull = std::path::PathBuf::from("/dev/null");
        let hash = get_hash(&devnull).unwrap();
        assert_eq!(hash, "HGbCilcGWPh");
    }

    #[test]
    fn get_hash_file() {
        let mut tempfile = tempfile::NamedTempFile::new().unwrap();
        tempfile.write_all(b"Hello, World!").unwrap();
        let hash = get_hash(&tempfile.path().to_path_buf()).unwrap();
        assert_eq!(hash, "40tfDDpk1nx");
    }

    #[test]
    fn make_hashed_tempdirname_devnull() {
        let devnull = std::path::PathBuf::from("/dev/null");
        let tempdir = make_hashed_tempdirname(&devnull, "test_").unwrap();
        assert_eq!(tempdir.to_string_lossy(), "/tmp/test_HGbCilcGWPh");
        assert!(! tempdir.exists());
    }
}
