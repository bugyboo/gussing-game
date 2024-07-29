use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    loop {

        println!("Gusse the number! (1 .. 10) - type 'quit' to exit");

        let secret_number = rand::thread_rng().gen_range(1..=10);

        let mut should_quit = false;

        let mut guess_again = true;

        while guess_again && !should_quit {        

            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)     
                .and_then(|_| {

                    match guess.trim().parse::<i32>() {
                        Ok(num) => {
                            println!("You guessed: {}", guess);
                            match num.cmp(&secret_number) {
                                Ordering::Less => println!("\x1b[33m xxxx Too small! xxxx\x1b[0m\n"),
                                Ordering::Greater => println!("\x1b[34m xxxx Too big! xxxx\x1b[0m\n"),
                                Ordering::Equal => { 
                                    println!("\x1b[32m=====> You win! <=====\x1b[0m\n\r");
                                    guess_again = false;
                                },
                            }
                        },
                        Err(_) => if guess.trim() == "quit" {
                            should_quit = true;
                        } else {
                            println!("Please enter a number!");
                        },                
                    }

                    Ok(())
                })
                .expect("Failed to read line");
        }

        if should_quit {
            println!("You Quit!");
            break;
        }

    }

}
