// This is a simple guessing game in Rust.
// The player has to guess a number between 1 and 100.
// The program generates a random number and gives feedback on the player's guess.

// Get the random number generator and input/output libraries 
use rand::Rng; 
use std::cmp::Ordering;
use std::io;

// The main function is the entry point of the program
fn main() {
    println!("Guess the number!"); // Print a welcome message using the println! macro

    // Generate a random number between 1 and 100
    // The rand::thread_rng() function creates a random number generator
    // The gen_range(1..=100) method generates a random number in the range 1 to 100 (inclusive)
    // The ..= syntax is a range that includes both endpoints
    // The secret_number variable is of type u32 (unsigned 32-bit integer)
    // The rand crate is used for random number generation
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Create a loop that will run until the player guesses the correct number
    // The loop keyword creates an infinite loop
    // The loop will continue until the break statement is reached
    // The break statement is used to exit the loop
    loop {
        println!("Please input your guess."); // Prompt the player to enter their guess

        let mut guess = String::new(); // Create a mutable variable guess of type String
        // The String::new() function creates a new empty string
        // The guess variable will store the player's input

        // The io::stdin() function gets the standard input stream
        // The read_line() method reads a line of input from the standard input stream
        // The &mut guess argument is a mutable reference to the guess variable
        // The expect() method is used to handle errors
        // If the read_line() method fails, the program will panic and print the error message
        // The read_line() method appends the input to the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // The trim() method removes whitespace from the beginning and end of the string
        // The parse() method converts the string to a number
        // The Ok(num) pattern matches the result of the parse() method
        // If the parse() method succeeds, num is the parsed number
        // If the parse() method fails, the Err(_) pattern matches the error
        // The continue statement skips the rest of the loop and starts the next iteration
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print the player's guess
        println!("You guessed: {guess}");

        // Compare the player's guess with the secret number
        // The cmp() method compares two numbers and returns an Ordering value
        // The Ordering enum has three variants: Less, Greater, and Equal
        // The match statement is used to handle the different cases of the comparison
        // If the guess is less than the secret number, print "Too small!"
        // If the guess is greater than the secret number, print "Too big!"
        // If the guess is equal to the secret number, print "You win!" and break the loop
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