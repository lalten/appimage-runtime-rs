use anyhow::{bail, Result};
use nix;
use std::path::PathBuf;

pub const SQUASHFUSE_DATA: &[u8] = include_bytes!(std::env!("COMPILE_DATA_PATH"));

pub fn squashfuse_mount<'a>(
    squashfs: &PathBuf,
    fs_offset: u64,
    mountpoint: &'a PathBuf,
) -> Result<&'a PathBuf> {
    std::fs::create_dir_all(mountpoint)?;

    // prepare squashfuse notify pipe
    let pipe_tmpdir = tempfile::tempdir()?;
    let notify_pipe = pipe_tmpdir.path().join("notify.pipe");
    nix::unistd::mkfifo(&notify_pipe, nix::sys::stat::Mode::S_IRWXU)?;

    // spawn the squashfuse driver
    let mut squashfuse_exec = memfd_exec::MemFdExecutable::new("squashfuse", SQUASHFUSE_DATA);
    squashfuse_exec
        .arg(format!("-ooffset={fs_offset}"))
        .arg(format!("-onotify_pipe={}", &notify_pipe.to_string_lossy()))
        .arg("-oauto_unmount")
        .arg("-otimeout=1")
        .arg(squashfs)
        .arg(mountpoint)
        .stdout(memfd_exec::Stdio::piped())
        .stderr(memfd_exec::Stdio::piped());
    let prog = &squashfuse_exec.get_argv().clone();
    let mut squashfuse_child = squashfuse_exec.spawn()?;

    // wait for squashfuse notify pipe to become readable or squashfuse to exit
    let mut notify_pipe_file = std::fs::File::open(&notify_pipe)?;
    let mut select = selecting::Selector::new();
    select.add_read(&notify_pipe_file);
    while !select
        .select_timeout(std::time::Duration::from_millis(10))?
        .is_read(&notify_pipe_file)
    {
        if let Some(status) = &mut squashfuse_child.try_wait()? {
            bail!("{prog:?} exited with status {status:?}");
        }
    }

    // squashfuse notify pipe is now readable
    let mut squashfuse_status = std::string::String::new();
    use std::io::Read;
    notify_pipe_file.read_to_string(&mut squashfuse_status)?;
    if squashfuse_status != "s" {
        let output = squashfuse_child.wait_with_output()?;
        bail!("{prog:?} notify-piped {squashfuse_status:?}:. {output:?}");
    }

    // squashfuse has daemonized at this point.
    Ok(mountpoint)
}
