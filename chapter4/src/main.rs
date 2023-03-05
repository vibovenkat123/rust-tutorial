fn main() {
    // scope
    { // start of scope
       let s = "s"; // can use s from now on
       println!("{}", s);
    }
    // s is not valid anymore
    // the String type can be mutable
    let mut s = String::from("s");
    s.push_str(" is mutable");
    println!("{}", s);
    // you can deallocate a variable
    drop(s);
    // error thrown
    //println!("{}", s);
}
