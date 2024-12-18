// guess higher or lower
use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line.");
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}

// // crates
// use rand::prelude::*;

// fn main() {
//     let number = random::<f64>();
//     println!("number is {}", number);

//     let number = thread_rng().gen_range(1..11);
//     println!("number is {}", number);
// }

// // parse string
// use std::io;

// fn main() {
//     let mut buffer = String::new();
//     println!("Enter a message:");
//     io::stdin().read_line(&mut buffer);

//     let number: i32 = buffer.trim().parse().unwrap();
//     println!("number + 1 is {}", number + 1);
// }

// // std input
// use std::io;

// fn main() {
//     let mut buffer = String::new();
//     println!("Enter a message:");
//     io::stdin().read_line(&mut buffer);
//     println!("buffer is {}", buffer);
// }