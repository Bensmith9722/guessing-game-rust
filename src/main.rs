
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number (Between 1..100)!");
    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = guess
                     .trim()
                     .parse()
                     .expect("Please type a number!");
                     
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => print!("Too small!"),
        Ordering::Greater => print!("Too big!"),
        Ordering::Equal => print!("You win!"),
    }
}
