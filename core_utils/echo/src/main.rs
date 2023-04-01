use std::{env, io::Write};
fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => {
            println!("")
        }
        2 => {
            println!("{}", args[1])
        }
        3 => {
            if args[1] == "-n" {
                print!("{}", args[2]);
            } else if args[2] == "-n" {
                print!("{}", args[1]);
            } else {
                println!("Too much arguments");
            }
        }
        _ => {
            println!("Cannot read system args");
        }
    }
    std::io::stdout().flush().unwrap();
}
