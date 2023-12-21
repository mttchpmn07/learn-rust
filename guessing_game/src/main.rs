use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number between 1 and 100 (inclusive)!");
    //println!("The secret number is {secret_number}.");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {guess}");
    
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,                
            };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
