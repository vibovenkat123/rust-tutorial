struct Config<T> {
    description: T,
}
fn main() {
    let string_conf = Config {description: "hello" };
    let number_conf = Config {description: 1};
}
