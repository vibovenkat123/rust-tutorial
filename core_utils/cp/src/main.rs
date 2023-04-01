use std::{fs, env, process::exit, cmp::Ordering};
fn main() {
    let args: Vec<String> = env::args().collect();
    let (src, dest): (String, String) = match args.len().cmp(&3) {
        Ordering::Less => {
            println!("Not enough arguments");
            exit(1);
        }
        Ordering::Greater => {
            println!("Too much arguments");
            exit(1);
        }
        Ordering::Equal=> {
            let arg1 = &args[1];
            let arg2 = &args[2];
            (arg1.to_string(), arg2.to_string())
        }
    };
    let copy_result = fs::copy(src, dest);
    let _ = match copy_result {
        Ok(val) => {
           val
        }
        Err(_) => {
            println!("File doesn't exist");
            exit(1);
        }
    };
}
