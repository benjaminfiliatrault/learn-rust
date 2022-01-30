use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess() {
    println!("Guess a number from 1 to 100!");

    let secret_number = rand::thread_rng().gen_range(1..101); // or 1..=100

    // Infinite looping
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing the previous guess variable to convert it to a number
        // if the user input is not a number, the program will re-ask for a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // If the user input is not a number, the program will re-ask for a number
            // because continue will return to the beginning of the loop
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
