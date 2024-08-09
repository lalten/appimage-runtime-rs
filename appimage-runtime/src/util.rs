use std::{ffi::OsString, vec::Vec};

pub fn get_appimage_path() -> std::path::PathBuf {
    let path = std::env::var("TARGET_APPIMAGE").unwrap_or("/proc/self/exe".to_string());
    let prog = std::path::PathBuf::from(path);
    match prog.is_relative() {
        true => {
            let bwd = std::env::var("BUILD_WORKING_DIRECTORY").unwrap_or(".".to_string());
            std::path::PathBuf::from(bwd).join(prog)
        }
        false => prog,
    }
}

pub fn get_elf_size(path: &std::path::PathBuf) -> u64 {
    use elf::endian::AnyEndian;
    use elf::ElfStream;
    let io = std::fs::File::open(path).expect(&format!("Failed to open file: {:?}", path));
    let elf = ElfStream::<AnyEndian, _>::open_stream(io).expect("Open ElfStream");
    let sht_end = elf.ehdr.e_shoff + elf.ehdr.e_shentsize as u64 * elf.ehdr.e_shnum as u64;
    let last_section = elf.segments().last().unwrap();
    let last_section_end = last_section.p_offset + last_section.p_filesz;
    std::cmp::max(sht_end, last_section_end)
}

pub fn print_help(argv0: &String) {
    println!(
        "appimage-runtime-rs: A type-2 AppImage runtime implementation in Rust, built with Bazel."
    );
    println!("");
    println!("Usage: {argv0} [OPTION] [ARG]...");
    println!("");
    println!("Options listed here will be consumed by the runtime. All other options will be passed to the application.");
    println!("");
    // println!("--appimage-extract-and-run      Extract content from embedded filesystem image and execute the AppRun");
    // println!("--appimage-extract [<pattern>]  Extract content from embedded filesystem image. If pattern is passed, only extract matching files");
    println!("--appimage-help   Print this help");
    println!("--appimage-mount  Mount embedded filesystem image and print mount point. Stop with Ctrl-C.");
    println!("--appimage-offset Print byte offset to start of embedded filesystem image");
    println!("");
    println!("It is an error to pass more than one of the above options.");
    println!("It is an error to pass a --appimage-* option not listed above.");
}

pub fn consume_appimage_arg(args: &Vec<OsString>) -> (Option<OsString>, Vec<OsString>) {
    if args.is_empty() {
        return (None, args.clone());
    }
    let arg1 = args.first().unwrap().clone();
    if arg1.to_string_lossy().starts_with("--appimage-") {
        return (Some(arg1), args[1..].to_vec());
    }
    (None, args.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(args_out.as_slice(), &args_in[1..]);
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
    }}
