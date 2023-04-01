use chrono::prelude::*;
use std::{env, process::exit, str};
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut formatter = "%c";
    if args.len() >= 2 {
        let first_char = match args[1].chars().nth(0) {
            None => {
                println!("Too much arguments");
                exit(1)
            },
            Some(val) => val,
        };
        if first_char == '+' && args.len() == 2 {
            let bytes = &args[1].as_bytes();
            formatter = match str::from_utf8(&bytes[1..]) {
                Ok(val) => val,
                Err(e) => panic!("{}", e),
            };
        } else {
            println!("Too much arguments");
            exit(1)
        }
    }
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format(formatter).to_string());
}
