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

pub fn command(binary: &str) -> Child {
    let child = Command::new(binary).spawn().expect("Spawn failed");

    return child;
}

