extern crate libc;
extern crate unshare;

use std::env;
use std::process;

use unshare::{Command, Namespace};

fn main() {
    let stage = env::args().nth(1).expect("too few arguments");
    match stage.as_ref() {
        "run" => run(),
        "child" => child(),
        _ => panic!("bad command"),
    }
}

fn run() {
    let args = env::args().skip(2).collect::<Vec<String>>();

    println!("Running [{}] as {}", args.join(" "), process::id());

    let mut child = Command::new("/proc/self/exe")
                            .args(&["child"])
                            .args(&args)
                            .unshare(&[
                                Namespace::Uts,
                                Namespace::Pid,
                            ])
                            .spawn()
                            .expect(&format!("/proc/self/exe failed to start"));

    let _ecode = child.wait()
                      .expect("failed to wait on child");
}

fn child() {
    let cmd = env::args().nth(2).expect("missing command");
    let args = env::args().skip(3).collect::<Vec<String>>();

    println!("Running [{} {}] as {}", cmd, args.join(" "), process::id());

    unsafe {
        let hostname = "container";
        libc::sethostname(hostname.as_ptr() as *const i8, hostname.len());
    }

    let mut child = Command::new(&cmd)
                            .args(&args)
                            .unshare(&[Namespace::Uts])
                            .spawn()
                            .expect(&format!("{} failed to start", &cmd));

    let _ecode = child.wait()
                      .expect("failed to wait on child");
}
