use std::io;
use rand::Rng;

fn main() {
//colours to make pretty ha ha ha 
    
    let green = "\x1b[32m";
    let blue = "\x1b[34m";
    let reset = "\x1b[0m";

    println!("Hello. 
    Welcome to Cheyenne's RANDOM NUMBER GENERATOR! 
    ◇──◆──◇──◆   ◇──◆──◇──◆   ◇──◆──◇──◆
    When you're ready, please enter a number:");
    
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    let first_number: f64 = match first_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Your input is invalid! Please enter a number!");
            return;
        }
    };

    println!("Great! Your first number is: {}{}{}.  Now enter another number!", blue, first_number, reset);
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    let second_number: f64 = match second_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };


    println!("You entered: {}{}{} and {}{}{}", blue, first_number,reset, blue, second_number, reset);

    let min = first_number.min(second_number);
    let max = first_number.max(second_number);

    // random number generator
    let mut rng = rand::thread_rng();

    // Generate a random number in the range [min, max]
    let random_number = rng.gen_range(min..=max);
    let rounded_random = random_number.round();

    // Print the random number
    println!("A random number between {} and {} is: {}{}{}.", min, max, green, rounded_random, reset);
}

