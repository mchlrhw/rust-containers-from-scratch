use std::env;
use std::process::Command;

fn main() {
    let stage = env::args().nth(1).expect("too few arguments");
    match stage.as_ref() {
        "run" => run(),
        _ => panic!("bad command"),
    }
}

fn run() {
    let cmd = env::args().nth(2).expect("missing command");
    let args = env::args().skip(3).collect::<Vec<String>>();

    println!("Running [{} {}]", cmd, args.join(" "));

    Command::new(&cmd)
            .args(args)
            .spawn()
            .expect(&format!("{} failed to start", &cmd));
}
