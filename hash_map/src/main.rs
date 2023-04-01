use std::collections::HashMap;
fn setup_highscore(name: String, prs: &mut HashMap<String, i32>) {
    prs.entry(name).or_insert(0);
}
fn main() {
    let mut prs: HashMap<String, i32> = HashMap::new();
    prs.insert(String::from("john"), 100000);
    prs.insert(String::from("bob"), 500);
    setup_highscore("john".to_string(), &mut prs);
    setup_highscore("tim".to_string(), &mut prs);
    let john_pr = prs.get("john").copied().unwrap_or(0);
    let tim_pr = prs.get("tim").copied().unwrap_or(0);
    println!("{}", john_pr);
    println!("{}", tim_pr);
}
