use std::io;

fn main() {
    println!("Gusse the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)     
        .and_then(|_| {
            println!("You guessed: {}", guess);
            Ok(())
        })
        .expect("Failed to read line");

}
