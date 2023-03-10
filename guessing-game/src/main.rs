// input/output library (analagous to <iostream>?)
use std::io;
// random number generator library
// Rng is a trait(?) that has to be in scope for us to use its methods
use rand::Rng;
// Brings the "Ordering" type into scope
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    // rand::thread_rng() gives us a random number generator local to the current thread of execution
    // gen_range() takes in a range expression and... generates a number in that range.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("secret number: {secret_number}");

    loop {
        println!("input your guess:");

        // variables have to be specifically marked mutable.
        // bind guess to a new string object
        let mut guess = String::new();

        // read input from stdin (like std::cin >> blah;)
        io::stdin() // this returns an instance of std::io::Stdin
            // append a line of user input to a mutable string (guess, in this case).
            // note that guess is passed by reference here, and the reference is made mutable with the mut keyword.
            .read_line(&mut guess)
            // error handling
            // .expect() is defined on the Result type, which can either be "Ok" or "Err".
            // If the Result is "Err", then the message we pass here is output and the program errors.
            .expect("Failed to read line");

        // try and parse guess as a u32 (unsigned 32-bit number) by annotating the type,
        // by using string functions to parse the previous value of guess.
        // then, assign the value to a new variable that shadows guess.
        // we can also use a match statement with the output of expect to just ignore invalid input.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // the "_" is a wildcard; catches all possible values of Err
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // use guess.cmp() to compare guess to the secret_number,
        // and use the match statement to match the types of orderings we could get as a result.
        // however, guess is a string and secret_number is a number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
