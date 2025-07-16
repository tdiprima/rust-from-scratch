use std::io;  // For input
use rand::Rng;  // Add to Cargo.toml: rand = "0.8.5", then cargo build

fn main() {
    println!("Guess the number (1-100)!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
