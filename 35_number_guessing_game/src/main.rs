#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

use rand::prelude::*;
use std::io::stdin;

fn main() {
    let mut rng = thread_rng();
    let number: i64 = rng.gen_range(1..100);

    loop {
        println!("Enter your guess: ");
        
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                // we trim the end for charaters like line break
                // then we parse with 64bit integer
                let parsed = buffer.trim_end().parse()::<i64>;
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!!!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e);
                    }
                }
            },
            Err(_) => continue,
        }
    }
}
