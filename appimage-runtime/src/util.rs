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
