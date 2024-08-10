use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SquashfuseErrorKind {
    Errno,
    Io,
    Process,
    NotifyPipe,
}

#[derive(Debug)]
pub struct SquashfuseError {
    kind: SquashfuseErrorKind,
    message: String,
}

impl fmt::Display for SquashfuseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{kind:?}: {message}",
            kind = self.kind,
            message = self.message
        )
    }
}

impl From<std::io::Error> for SquashfuseError {
    fn from(error: std::io::Error) -> Self {
        SquashfuseError {
            kind: SquashfuseErrorKind::Io,
            message: error.to_string(),
        }
    }
}

impl From<nix::errno::Errno> for SquashfuseError {
    fn from(error: nix::errno::Errno) -> Self {
        SquashfuseError {
            kind: SquashfuseErrorKind::Errno,
            message: error.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, SquashfuseError>;

pub const SQUASHFUSE_DATA: &[u8] = include_bytes!(std::env!("COMPILE_DATA_PATH"));

pub fn squashfuse_mount(
    squashfs: &std::path::PathBuf,
    fs_offset: u64,
) -> Result<std::path::PathBuf> {
    let mountpoint = mkdtemp_mountpoint();

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
        .arg(squashfs)
        .arg(&mountpoint)
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
            return Err(SquashfuseError {
                kind: SquashfuseErrorKind::Process,
                message: format!("{prog:?} exited with status {status:?}").to_string(),
            });
        }
    }

    // squashfuse notify pipe is now readable
    let mut squashfuse_status = std::string::String::new();
    use std::io::Read;
    notify_pipe_file.read_to_string(&mut squashfuse_status)?;
    if squashfuse_status != "s" {
        let output = squashfuse_child.wait_with_output()?;
        return Err(SquashfuseError {
            kind: SquashfuseErrorKind::NotifyPipe,
            message: format!("{prog:?} notify-piped {squashfuse_status:?}:. {output:?}")
                .to_string(),
        });
    }

    // squashfuse has daemonized at this point.
    Ok(mountpoint)
}

fn mkdtemp_mountpoint() -> std::path::PathBuf {
    let template = std::ffi::CString::new("/tmp/.mount_XXXXXX")
        .unwrap()
        .into_raw();
    let mountpoint_c = unsafe { nix::libc::mkdtemp(template) };
    let mountpoint = std::path::PathBuf::from(
        unsafe { std::ffi::CStr::from_ptr(mountpoint_c) }
            .to_string_lossy()
            .to_string(),
    );
    mountpoint
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mountpoint_creation() {
        let mountpoint = mkdtemp_mountpoint();
        assert!(!mountpoint.to_string_lossy().ends_with("XXXXXX"));
        assert!(mountpoint.is_dir());
    }
}
