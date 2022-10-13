use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

#[allow(dead_code)]
pub fn run() {
    let collision_emoji = '\u{1F4A5}'; // 💥
    let smiling_face = '\u{1F601}'; // 😁
    let lovely_face = '\u{1F970}'; // 🥰
    let thinking_face = '\u{1F914}'; // 🤔
    let worried_face = '\u{1F61F}'; // 😟
    let sad_face = '\u{1F614}'; // 😔

    // TODO: let user choose the number of lives in game
    let mut lives = 5;

    // * the randomly generated number
    let secret_number = rand::thread_rng().gen_range(1..101);

    // game starts here
    println!("> Welcome to the guessing game {}", smiling_face);
    println!(">");
    println!("> Guess a number between {{1 - 100}} below... {}", thinking_face);

    loop {
        if lives == 0 {
            println!("> {} {}", "Game over!".red(), sad_face);
            break;
        }

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        lives -= 1;

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("> {}? Your input is too big!! You have {} lives remaining {}", guess, lives, worried_face),
            Ordering::Less => println!("> {}? Your input is too small.. You have {} lives remaining {}", guess, lives, worried_face),
            Ordering::Equal => {
                println!("> {} Wohooooo you won {} {}", "You made it!".green(), lovely_face, collision_emoji);
                break;
            }
        }
    }
}
