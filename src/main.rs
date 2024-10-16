use rand::Rng; // Generator of random numbers //
use std::io; // Read the keyboard //

// Function to show the user the options, read the choice and return to the main program
fn choose_game_mode() -> char {
    let mut game_option_string = String::new();
    // Read the value of the keyboard (String type) and transform to number (Integer type)
    io::stdin()
        .read_line(&mut game_option_string)
        .expect("Failed to read the line.");
    let game_option_number: char = game_option_string
        .trim()
        .parse()
        .expect("Failed to read a valid number.");
    game_option_number // Return the valid number to be used in the main program
}

// Function to choose a random word to play the game: from 10 words predefined
// Tip for future improvements: read a external text arquive with the words
// Tip for future improvements: indicate a tip to help the user (fruit, animal, objects, etc.)
fn choose_word() -> String {
    let random_num: usize = rand::thread_rng().gen_range(0..9); // Generate a random number to index a random word to play
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
    let word_to_play: String = (words_to_play[random_num]).to_string();
    word_to_play // Return a random word to play the game
}

// Function to verify the number of characters of the random word and create a mask to shadow the word
fn mask_word(word_to_play: &String) -> String {
    let number_characters: usize = word_to_play.chars().count();
    let mut mask_word_characters: String = String::new();
    for _ in 0..number_characters {
        mask_word_characters.push('_');
    }
    mask_word_characters // Return the word masked to show to the user
}

// Function to read the user attempt
fn read_user_attemp() -> char {
    println!("- Type in the suggested letter");
    println!("- Type the number 1 to quit game");
    let mut letter_attemp = String::new();
    io::stdin()
        .read_line(&mut letter_attemp)
        .expect("Failed to read an alphanumeric");
    let letter_attemp = letter_attemp.trim(); // Convert String into &str and remove the Enter command
    let letter_char: char = letter_attemp.chars().next().unwrap(); // Convert &str to char
    letter_char // Return a char with user attemp
}

// Function to manage user attempts
fn manage_attemps(word: &String, mask: &mut String) {
    // Maximum number of attemps for the user find the word, stay on this loop until the end of the game
    let mut attemps: i32 = 10;
    let mut letters: String = String::new();
    loop {
        let user_choice = read_user_attemp();
        match user_choice {
            '1' => break,
            _ => {
                if word.contains(user_choice) {
                    mask.push(user_choice);
                    letters.push(user_choice);
                } else {
                    attemps -= 1;
                    letters.push(user_choice);
                }
            }
        }
        let masked: String = format_masked_string(word, &mask);
        clearscreen::clear().unwrap();
        if !masked.contains('_') {
            clearscreen::clear().unwrap();
            println!("Congratulations, you have discovered the secret word:");
            println!("############### {} ###############", masked);
            break;
        } else {
            if attemps == 0 {
                clearscreen::clear().unwrap();
                println!("!!!! You couldn't find the secret word: {} !!!!", word);
                break;
            } else {
                println!("# Masked word: ### {} ###", masked);
                println!("# All the letters tried: {}", letters);
                println!("# Remaining attemps: {}", attemps);
            }
        }
    }
}

fn format_masked_string(input: &String, mask: &String) -> String {
    let mut result: String = String::new();
    for (_, c) in input.chars().enumerate() {
        println!("{}", result);
        if mask.contains(c) {
            result.push(c);
        } else {
            result.push('_');
        }
    }
    result // returns the masket word displaying the correct letters
}

// Function to play the game
fn play_game() {
    clearscreen::clear().unwrap();
    let word_to_play = choose_word();
    let mut masked_word = mask_word(&word_to_play);
    println!("# Masked word: ### {} ###", masked_word);
    manage_attemps(&word_to_play, &mut masked_word);
}

// Function to show info to the user
fn game_info() {
    clearscreen::clear().unwrap();
    println!("------------------------------------------------------------------------------");
    println!("Hangman is a guessing game for one or more players.");
    println!("The game chooses a random word and displays the number of characters");
    println!("The player must type one letter at a time until they find the complete word");
    println!("The player can miss up to 10 attempts");
    println!("------------------------------------------------------------------------------");
}

fn main() {
    println!("-------------------------------------------");
    println!("  Welcome to the Hangman's Game in Rust");
    println!("Personal challenge to test language skills");
    println!("   Developed by Matheus Eduardo Aver");
    println!("        matheusaver@gmail.com");
    println!("-------------------------------------------");
    // Show the options until a valid option is chosen
    loop {
        println!("Choose the option you want:");
        println!("1 - Play");
        println!("2 - Info");
        println!("3 - Exit");
        let game_option = choose_game_mode();
        match game_option {
            '1' => {
                play_game();
            }
            '2' => {
                game_info();
                continue;
            }
            '3' => break,
            _ => {
                clearscreen::clear().unwrap();
                println!("Invalid Option, choose another and try again.");
            }
        }
    }
}
