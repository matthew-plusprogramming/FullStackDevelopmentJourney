use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number: {}", secret_number);

    loop {
        let mut guess = String::new();
        println!("Please guess your number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed less"),
            Ordering::Equal => {
                println!("You guessed equal");
                break;
            }
            Ordering::Greater => println!("You guessed greater"),
        }
    }
}
