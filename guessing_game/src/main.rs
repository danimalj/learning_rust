//This imports the libraries much like python. 
//The `std` library is the standard library that comes with Rust. 
//The `io` library is the input/output library that comes with Rust.
use std::io;
use rand::Rng;
use std::cmp::Ordering;

//running the main function
fn main() {
    //The `let` keyword is used to create a new variable.
    //The `mut` keyword is used to make a variable mutable.
    
    //println! is a macro that prints a string to the screen
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                        .gen_range(1..=1000);


    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
    
        //The `io` library has a function called `stdin` that 
        //returns a handle to the standard input for your terminal.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
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
