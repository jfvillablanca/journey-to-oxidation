use std::io;

fn main() {
    println!("Guess the number hehe!");
    println!("Kindly enter your guess here.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");

    println!("You guessed: {}", guess);
}
