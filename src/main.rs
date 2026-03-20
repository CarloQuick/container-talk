use anyhow::{Context, Result};
use dotenv::dotenv;
use nix::{
    mount::{MsFlags, mount},
    sched::{CloneFlags, unshare},
    sys::wait::waitpid,
    unistd::{ForkResult, chroot, execvp, fork, getcwd, gethostname, getpid, getuid, sethostname},
};
use std::env;
use std::{ffi::CString, path::PathBuf};

fn print_proc_info(label: &str) -> Result<()> {
    eprintln!("[{}]", label);
    eprintln!(
        "uid [{}]\n\thostname [{:?}]  \n\tpid [{}] \n\tcwd [{:?}]",
        getuid(),
        gethostname()?,
        getpid(),
        getcwd()?,
    );
    Ok(())
}
fn write_proc_mappings(uid_map: &str, gid_map: &str) -> Result<()> {
    std::fs::write("/proc/self/uid_map", uid_map)?;
    std::fs::write("/proc/self/setgroups", "deny")?;
    std::fs::write("/proc/self/gid_map", gid_map)?;
    Ok(())
}
fn child(container_dir: &PathBuf, rootfs: &PathBuf, argv: &[CString]) -> Result<()> {
    // === UTS NAMESPACE ===
    unshare(CloneFlags::CLONE_NEWUTS).context("Failed to isolate uts namespace")?;
    sethostname("my-container")?;

    // === MOUNT NAMESPACE ===
    unshare(CloneFlags::CLONE_NEWNS).context("Failed to isolate mount namespace")?;
    mount(
        Some(rootfs),
        container_dir,
        None::<&str>,
        MsFlags::MS_BIND,
        None::<&str>,
    )
    .context("Failed to mount rootfs")?;

    // Change the root directory of the process
    chroot(container_dir).context("Failed to change the root dir")?;

    // Set the process' current working directory
    std::env::set_current_dir("/")?;
    print_proc_info("Container Isolation")?;
    execvp(&argv[0], argv)?;
    Ok(())
}
fn main() -> Result<()> {
    dotenv().ok();
    let rootfs = PathBuf::from(env::var("ROOT_FS")?);
    let container_dir = PathBuf::from(env::var("CONTAINER_DIR")?);

    print_proc_info("Before Isolation")?;

    // === USER NAMESPACE ===
    let uid_map = format!("0 {} 1", nix::unistd::getuid());
    let gid_map = format!("0 {} 1", nix::unistd::getgid());
    unshare(CloneFlags::CLONE_NEWUSER)?;
    write_proc_mappings(&uid_map, &gid_map)?;

    // === PID NAMESPACE - next forked child will be PID 1 ===
    unshare(CloneFlags::CLONE_NEWPID)?;
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None)?;
        }
        Ok(ForkResult::Child) => {
            let argv = vec![CString::new("ls")?];
            child(&container_dir, &rootfs, &argv)?;
        }
        Err(err) => Err(err).context("Fork failed!")?,
    }
    Ok(())
}
