use std::io;

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");

    println!("Input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read the line");

    println!("You've guesses: {guess}");
}
