use std::env;

fn main() {
    let stage = env::args().nth(1).expect("too few arguments");
    match stage.as_ref() {
        "run" => run(),
        _ => panic!("bad command"),
    }
}

fn run() {
    let cmd = env::args().skip(2)
                         .collect::<Vec<String>>()
                         .join(" ");
    println!("Running {}", cmd);
}
