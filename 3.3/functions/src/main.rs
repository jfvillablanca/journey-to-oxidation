fn main() {
    println!("Hello, world!");
    new_func(69, '😎');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn new_func(x: u8, emoji: char) {
    println!("I like {x}. I am {emoji}");
}
