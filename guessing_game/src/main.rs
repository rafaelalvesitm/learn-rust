use std::io; // Import the standard input/output library

fn main() { 
    println!("Guess the number!");

    println!("Please input your guess.");

    // This line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
    let mut guess = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}