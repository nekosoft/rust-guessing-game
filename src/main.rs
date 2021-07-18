use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101); // guessable range 1-100, gen_range is inclusive of lower bound but exclusive on upper bound

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess) // stores type io::Result
            .expect("Failed to read line"); // error handling, crashes the program

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch all value
        };

        println!("You guessed: {}", guess); // {} = placeholder for a value, many can be used

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
