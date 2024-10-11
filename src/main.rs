use rand::Rng;
use std::io; // Read the keyboard // Generator of random numbers

// Function to show the user the options, read the choice and return to the main program
fn choose_game_mode() -> i32 {
    let mut game_option_string = String::new();
    println!("Choose the option you want:");
    println!("1 - Play");
    println!("2 - Info");
    println!("3 - Exit");
    // Read the value of the keyboard (String type) and transform to number (Integer type)
    io::stdin()
        .read_line(&mut game_option_string)
        .expect("Failed to read the line.");
    let game_option_number: i32 = game_option_string
        .trim()
        .parse()
        .expect("Failed to read a valid number.");
    game_option_number // Return the valid number to be used in the main program
}

// Function to play the game
fn play_game() {
    clearscreen::clear().unwrap();
    println!("Function to play the game");
    let random_num: usize = rand::thread_rng().gen_range(0..9); // Generate a random number to index a random word to play
    println!("Random number generated: {}", random_num);
    // Words predefined to play the game
    let words_to_play = vec![
        "banana".to_string(),
        "apple".to_string(),
        "melon".to_string(),
        "papaya".to_string(),
        "orange".to_string(),
        "lemon".to_string(),
        "grape".to_string(),
        "coconut".to_string(),
        "mango".to_string(),
        "peach".to_string(),
    ];
    println!("Random word to play: {}", words_to_play[random_num]);
    // Verify the number of characters of the random word and create a mask to shadow the word to play
    let number_characters: usize = words_to_play[random_num].chars().count();
    let mut mask_word_characters: String = String::new();
    for _ in 0..number_characters {
        mask_word_characters.push('_');
        mask_word_characters.push(' ');
    }
    println!("Word masked: {}", mask_word_characters);
    // Maximum number of attemps for the user find the word, and the attemps remaining
    let attemps_max: i32 = 10;
    let mut letter_attemp: String = String::new();
    for _ in 0..attemps_max {
        println!("Type in the suggested letter:");
        io::stdin()
        .read_line(&mut letter_attemp)
        .expect("Failed to read the letter.");
    }
 
    
}

// Function to show info to the user
fn game_info() {
    clearscreen::clear().unwrap();
    println!("Function to show info to the user");
}

fn main() {
    println!("-----------------------------------------");
    println!("Welcome to the Hangman's Game in Rust");
    println!("-----------------------------------------");
    // Show the options until a valid option is chosen
    loop {
        let game_option = choose_game_mode();
        match game_option {
            1 => {
                play_game();
                break;
            }
            2 => {
                game_info();
                break;
            }
            3 => break,
            _ => {
                clearscreen::clear().unwrap();
                println!("Invalid Option, choose another.");
            }
        }
    }
}
