use rand::Rng; // the randomizer lib from Rust. 
use std::cmp::Ordering; // the ordering and comparation lib from Rust.
use std::io; // the input/output lib from Rust. 
    //use is the command to appoint the imports libs of your code.

fn main() {
    println!("Guess the Number!!");

    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);
    let mut guesses:u32 = 0;
    // a unassigned 32bit variable (positive integer number) that receives a fixed random number for each execution of the program, with the range 1 to 100. 

    loop {
        println!("Please, type your guess: ");
        let mut user_guess: String = String::new(); 
            // let is the prefix to a new variable, binding the value.
            // mut is a mutable variable. The name "variable" appoints to variability. But Rust distinguishes between mutable and immutable.
            // String its the type of our variable. we have &str as another example, but the difference is the control of the information.
            // ::new() the "::" it's how we can call a function in Rust. New is a Function of String, that creates a new empty String. 

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read your input.");
            // stdin() its the function from std::io that receives the input from the user.
            // .read_line() its where you appoint the variable that will receive the input
            // .expect() represents the "fail" situation, where you can set a message for user.

        let user_guess: u32 = match user_guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        // in this case, we are using "match" to run the code conditionally, if we parse successfully, the variable you be converted to u32.

        guesses+= 1;
        // here add + 1 in how many times we have tried.

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        //here, the "match" is used to run code by 3 patterns with 3 respective actions.
    }

    println!("You've tried {guesses} times!")
}
