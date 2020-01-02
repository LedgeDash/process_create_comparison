use nix::unistd::{self, Pid, ForkResult};
use std::ffi::{CString};
use std::process::{Command, Child};
use libc;

/// use UNIX fork() and exec() to create a new process
pub fn fork_exec(binary: &str) -> Pid {

    let s = CString::new(binary).expect("CString::new failed");
    match unistd::fork() {
        Err(_) => {
            println!("Fork failed");
            return Pid::from_raw(-1);
        }
        Ok(ForkResult::Parent {child, ..}) => {
            return child;
        }
        Ok(ForkResult::Child) => {
            match unistd::execv(&s, &[]) {
                Ok(_) => return unistd::getpid(),
                Err(_) => panic!("execv failed"),
            }
        }
    }
}

/// vfork() Deprecated since 0.2.66: causes memory corruption, see rust-lang/libc#1596
pub fn vfork_exec(binary: &str) -> Pid {

    let s = CString::new(binary).expect("CString::new failed");
    match unistd::fork() {
        Err(_) => {
            println!("Fork failed");
            return Pid::from_raw(-1);
        }
        Ok(ForkResult::Parent {child, ..}) => {
            return child;
        }
        Ok(ForkResult::Child) => {
            match unistd::execv(&s, &[]) {
                Ok(_) => return unistd::getpid(),
                Err(_) => panic!("execv failed"),
            }
        }
    }
}

/// `std::process::Command` uses libc::posix_spawn to create child process
pub fn posix_spawn(binary: &str) -> Pid {

    let s = CString::new(binary).expect("CString::new failed");
    match unistd::fork() {
        Err(_) => {
            println!("Fork failed");
            return Pid::from_raw(-1);
        }
        Ok(ForkResult::Parent {child, ..}) => {
            return child;
        }
        Ok(ForkResult::Child) => {
            match unistd::execv(&s, &[]) {
                Ok(_) => return unistd::getpid(),
                Err(_) => panic!("execv failed"),
            }
        }
    }
}

/// create a process using the `std::process::Command` API
pub fn command(binary: &str) -> Child {
    let child = Command::new(binary).spawn().expect("Spawn failed");

    return child;
}

