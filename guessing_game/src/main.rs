use std::io; //IO from standard library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable, default let in rust are immutable by default
        // syntax :: mean new is an associated function of the String type
        // associated function is a function that’s implemented on a type
        io::stdin()
            .read_line(&mut guess) // & indicates that this argument is reference
            .expect("Failed to read line"); // .read_line and .expect are methods

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess

        println!("You guess: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    
}
