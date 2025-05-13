use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
        println!("Guess the number!"); /*println! is a macro that prints a message to the console.*/

        let secret_number = rand::thread_rng().gen_range(1..=100); /*Inclusive range*/


    loop { /*loop is used to repeat the code block until a condition is met*/
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) /*&mut guess is a reference to the guess variable*/
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { /*trim() removes any whitespace from the string, and the guess variable is now a u32 shadowing the mut guess variable from the String type. parse() converts the string to a u32.*/
            Ok(num) => num,
            Err(_) => continue, /*The underscore is a catchall value. In this case, it is used to ignore the error value returned by the parse() function.*/
        };
            println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){ /*cmp() compares the guess to the secret number*/
            Ordering::Less => println!("Too small!"), /*Each arm of the match expression is a pattern and a block of code. The match expression is evaluated from top to bottom, and the first pattern that matches is executed.*/
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; /*break is used to exit the loop*/
            }
        }
    }
}
/*If ever curious about the function of a fucntion, cargo doc --open command will build documentation for you*/