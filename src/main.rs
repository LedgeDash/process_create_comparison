use clap::*;
use nix::unistd::{Pid};
use process_create_compare::*;
use nix::sys::wait;
use std::process::{Command, Child};
use time::precise_time_ns;

fn main() {
    let cmd_arguments = App::new("process creation comparison experiment")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Launch a process creation experiment")
        .arg(
            Arg::with_name("parent memory")
                .short("m")
                .long("mem")
                .value_name("memory")
                .takes_value(true)
                .required(true)
                .help("Amount of allocated memory in parent process BEFORE creating processes"),
        )
        .arg(
            Arg::with_name("child count")
                .short("c")
                .long("count")
                .value_name("count")
                .takes_value(true)
                .required(false)
                .default_value("500")
                .help("The number of child processes created"),
        )
        .arg(
            Arg::with_name("child binary")
                .short("b")
                .long("binary")
                .value_name("binary")
                .takes_value(true)
                .required(true)
                .help("Path to child binary"),
        )
        .arg(
            Arg::with_name("creation method")
                .short("w")
                .long("method")
                .value_name("method")
                .takes_value(true)
                .required(true)
                .help(
                    "Process creation method.\n
                          - 0: fork-exec\n
                          - 1: vfork-exec\n
                          - 2: posix_spawn\n
                          - 3: std::process::Command\n",
                ),
        )
        .get_matches();

    let mem: usize = cmd_arguments
        .value_of("parent memory")
        .unwrap()
        .parse()
        .expect("Cannot convert to integer");
    let binary = cmd_arguments.value_of("child binary").unwrap();
    let count: usize = cmd_arguments
        .value_of("child count")
        .unwrap()
        .parse()
        .expect("Cannot convert count to integer");
    let method: usize = cmd_arguments
        .value_of("creation method")
        .unwrap()
        .parse()
        .expect("Cannot convert count to integer");
    let methods = vec![
        "fork-exec",
        "vfork-exec",
        "posix_spawn",
        "std::process::Command",
    ];

    println!("Parent memory: {:?}MB", mem);
    println!("Child binary: {:?}", binary);
    println!(
        "Creating {:?} child processes with {:?}",
        count, methods[method]
    );

    let size = mem*1024*1024/4; // number of u32
    let mut buf = Vec::<u32>::with_capacity(size);
    for i in 0..size {
        buf.push(i as u32);
    }


    let mut cpids: Vec<Pid> = vec![];
    let mut num_children = 0;
    let mut commands: Vec<Child> = vec![];

    let t1 = precise_time_ns();
    for _ in 0..count {
        let cpid = match method {
            0 => {
                fork_exec(binary)
            }
            1 => {
                vfork_exec(binary)
            }
            2 => {
                posix_spawn(binary)
            }
            3 => {
                let c = command(binary);
                commands.push(c);
                Pid::from_raw(-1)
            }
            _ => {
                panic!("Unknown process creation method");
            }
        };

        let cpid = cpid.as_raw();
        if cpid != -1 {
            cpids.push(Pid::from_raw(cpid));
            num_children = num_children+1;
        }
    }
    let t2 = precise_time_ns();

    if method != 3 {
        while num_children > 0 {
            let status = wait::wait();
            if let Ok(wait::WaitStatus::Exited(_, _)) = status {
                num_children = num_children-1;
            }
        }
    } else {
        for mut child in commands {
            child.wait().expect("Failed to wait for child");
        }
    }

    println!("{:?}", t2-t1);
}
