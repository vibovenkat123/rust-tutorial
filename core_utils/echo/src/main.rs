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
fn formatted(msg: Option<&str>) -> String {
    match msg {
        Some(n) => format!("{n}"),
        None => format!(""),
    }
}
