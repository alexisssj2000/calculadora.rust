use std::cmp::Ordering;
use rand::prelude::*;
use std::io;

fn main() {
    println!("‡--------------------------------→");
    println!("‡ Welcome to the Rust Calculator!");
    println!("‡--------------------------------→");
    println!("‡ Please Guess the number: ");
    
    let secret_number =String::from(&rand::rng().random_range(1..=100).to_string());
    
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    
    

    
    println!("‡ The secret number is: {}", secret_number);
    println!("‡ You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("‡ Too small!"),
        Ordering::Greater => println!("‡ Too big!"),
        Ordering::Equal => println!("‡ You win!"),
    }
}
