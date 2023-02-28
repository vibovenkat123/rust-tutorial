use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let lower_range = 100;
    let higher_range = 300;
    println!("You have been randomly selected to win the lottery!");
    println!("All you have to do is guess the number between {lower_range} and {higher_range}");
    let lotto_number = rand::thread_rng().gen_range(100..=300);
    let amount = 1_000_000_000;
    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed {guess}");
        match guess.cmp(&lotto_number) {
            Ordering::Less => println!("The number you entered was low, enter something higher"),
            Ordering::Equal => { 
                println!("Congratulations, you won ${amount} USD");
                break;
            },
            Ordering::Greater => println!("The number you entered was high, enter something lower")
        }
    }
}
