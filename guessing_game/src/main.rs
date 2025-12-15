use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Guess the number!");

        let mut guess = String::new(); // creating a new string variable

        io::stdin().read_line(&mut guess).expect("Failed to Read the input!");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
