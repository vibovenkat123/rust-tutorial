use std::{env, process::exit};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        exit(1);
    }
    if args.len() != 3 {
        println!("Usage: {} <input_string> <find_string>", args[0]);
        exit(1);
    }

    let input_string = &args[1];
    let find_string = &args[2];
    let lines = input_string.split("\n");

    for line in lines {
        if line.contains(find_string) {
            println!("{}", format!("\"{}\"", line));
        }
    }
}
