use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    const LOWER_BOUND: i8 = 1;
    const UPPER_BOUND: i8 = 100;
    println!(
        "Guess the secret number from {} to {}!",
        LOWER_BOUND, UPPER_BOUND
    );

    let secret_number = generate_number(LOWER_BOUND, UPPER_BOUND);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("** You win! **");
                break;
            }
        }
    }
    println!("Game complete!")
}

fn generate_number(lower_bound: i8, upper_bound: i8) -> i32 {
    let secret_number = rand::thread_rng().gen_range(lower_bound..=upper_bound);
    secret_number as i32
}
