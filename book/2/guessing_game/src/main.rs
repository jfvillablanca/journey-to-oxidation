use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number hehe!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("secret num: {secret_num}");

    loop {
        println!("Kindly enter your guess here.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");

        let guess: u32 = match guess
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("A tad bit too small."),
            Ordering::Greater => println!("Whoops! Too big."),
            Ordering::Equal => {
                println!("Congratulations! You guessed it.");
                break;
            }
        }
    }
}
