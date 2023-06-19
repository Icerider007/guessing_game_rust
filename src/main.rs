use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

   let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("The secret number is {secret_number}");
    
    println!("Guess the number!");
    println!("Enter your guess (Number b/w 1 and 100)");    
    
   loop{
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("A number between 1 and 100 please");
                continue;
            } 
        };
    
        println!("You guessed : {guess} ");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess higher!"),
            Ordering::Greater => println!("Guess lower!"),
            Ordering::Equal => {
               println!("That's right!!");
               break; 
            }
        } 
    } 
}
