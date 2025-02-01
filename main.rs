use std::io;
use rand::prelude::*;

fn main() {
    let list = ["pink", "red", "yellow", "orange", "green", "violet"];
    let mut rng = thread_rng();
    let index = rng.gen_range(0..list.len());
    let random_colour = list[index];

    loop {
        let mut input = String::new();
        println!("Guess the color:");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let colour = input.trim().to_lowercase();
                println!("Colour selected: {}", colour);

                if !list.contains(&colour.as_str()) {
                    println!("Wrong colour. Try again!");
                    continue;
                }

                if colour_checker(&colour, random_colour) {
                    println!("Correct guess!");
                    break;
                } else {
                    println!("Retry!");
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn colour_checker(guessed_colour: &str, random_colour: &str) -> bool {
    guessed_colour == random_colour
}
