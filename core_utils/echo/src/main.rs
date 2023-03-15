use std::env;
fn main() {
    let rawmessage = env::args().nth(1);
    match rawmessage {
        None => {
            println!("")
        }
        Some(x) => {
            println!("{}", x)
        }
    }
}
