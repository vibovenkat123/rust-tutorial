use std::{fs, env, process::exit, io, io::Write};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need file argument");
        exit(1)
    }
    if args.len() > 2 {
        println!("Too much arguments");
        exit(1)
    }
    let file_path = &args[1];
    let content_result = fs::read_to_string(file_path);
    let content = match content_result {
        Ok(file) => file,
        Err(_) => {
            println!("File not found");
            exit(1);
        }
    };
    print!("{}", content);
    io::stdout().flush().unwrap()
}
