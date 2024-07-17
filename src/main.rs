extern crate libc;
extern crate structopt;

use libc::kill;
use std::process::Command;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "process_monitor")]
struct Opt {
    /// The process ID to monitor
    #[structopt(short, long, parse(try_from_str = str::parse::<i32>))]
    pid: i32,

    /// The interval in milliseconds between checks
    #[structopt(short, long, default_value = "5000", parse(try_from_str = str::parse::<u64>))]
    interval: u64,
}

fn main() {
    let opt = Opt::from_args();

    if !is_process_running(opt.pid) {
        println!("process [{}] is not runing.", opt.pid);
        return;
    }

    println!("start listen process [{}]", opt.pid);

    loop {
        if !is_process_running(opt.pid) {
            println!("process [{}] is over, shutdown...", opt.pid);
            shutdown_system();
            break;
        }
        thread::sleep(Duration::from_millis(opt.interval));
    }
}

fn is_process_running(pid: i32) -> bool {
    unsafe {
        // Try sending signal 0 to the process.
        // This will not actually send any signal but check if the process exists.
        kill(pid, 0) != -1
    }
}

fn shutdown_system() {
    Command::new("shutdown")
        .arg("-h")
        .arg("now")
        .status()
        .expect("Failed to execute shutdown command");
}
