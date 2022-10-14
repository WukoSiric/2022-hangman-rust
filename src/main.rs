use std::{collections::HashSet}; //Storing correct letters
use rand::Rng; //Choosing random word
use std::io;

const WORD_LIST: [&str; 50] = [
    "apple", "banana", "carrot", "dog", "elephant", "fig", "grape", "honey", "ice", "jelly", "kiwi",
    "lemon", "mango", "nut", "orange", "peach", "quince", "raspberry", "strawberry", "tomato",
    "unicorn", "vanilla", "watermelon", "xenon", "yogurt", "zucchini", "aardvark", "bear", "cat",
    "dolphin", "eagle", "fox", "giraffe", "hippo", "iguana", "jaguar", "kangaroo", "lemur", "mouse",
    "newt", "octopus", "panda", "quail", "raccoon", "seal", "tiger", "urchin", "vulture", "wombat",
    "xerus"
];

fn choose_word(word_list: [&str; 50]) -> String {
    let rng = rand::thread_rng().gen_range(0..WORD_LIST.len());
    return word_list[rng].to_string().to_uppercase();
}

fn get_input() -> String {
    let mut guess = String::new();
    let mut guess_length = 0;

    while guess_length != 1 {
        guess = String::from("");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();
        guess_length = guess.len();
    }

    let guess = guess.to_uppercase();
    return guess
}

fn guess_letter(word: &str, letter: String) -> bool {
    let letter = letter.to_uppercase();
    return word.contains(letter.as_str())
}

fn update_user_letters(word: &str, letter: String, user_letters: &mut HashSet<String>) -> bool {
    let letter_exists: bool = guess_letter(word, letter.clone());
    if letter_exists {
        user_letters.insert(letter);
        return true
    }
    return false
}

fn get_correct_letters(word: &str) -> HashSet<String> {
    let mut letters: HashSet<String> = HashSet::new();
    for letter in word.chars() {
        letters.insert(letter.to_string());
    }
    return letters
}

fn print_word(word: &str, user_letters: &HashSet<String>) -> String {
    let mut output = String::new();
    for letter in word.chars() {
        if user_letters.contains(&letter.to_string()) {
            output.push(letter);
            output.push(' ');
        } else {
            output.push('_');
            output.push(' ');
        }
    }
    return output;
}

fn main() {
    let mut guesses = 0;
    const MAX_GUESSES: u32 = 6;
    let chosen_word = choose_word(WORD_LIST);
    let mut user_letters: HashSet<String> = HashSet::new();
    let correct_letters: HashSet<String> = get_correct_letters(&chosen_word);

    print!("\x1B[2J\x1B[1;1H"); //Clears the screen
    println!("Welcome to Hangman!");
    println!("A word has been chosen, it is {} letters long", chosen_word.len());

    while guesses < MAX_GUESSES {
        if user_letters.len() == correct_letters.len() {
            println!("Congradulations, you win!");
            println!("Thanks for playing :0");
            return
        }
        println!("Guess a letter");
        let guess = get_input();
        let letter_exists = update_user_letters(&chosen_word, guess, &mut user_letters);
        print!("\x1B[2J\x1B[1;1H"); //Clears the screen
        if letter_exists {
            println!("Correct!");
        } else {
            println!("Incorrect!");
            guesses += 1;
        }
        println!("You have {} guesses left", MAX_GUESSES - guesses);
        println!("{}", print_word(&chosen_word, &user_letters));
    }

    println!("You lose!");
    println!("The word was: {}", chosen_word);

}

#[test]
fn can_choose_word() {
    let chosen_word = choose_word(WORD_LIST); 
    assert_eq!(true, WORD_LIST.contains(&chosen_word.as_str()));
}

#[test]
fn can_guess_letter_correct() {
    let word = "APPLE";
    let guess = String::from("A"); 
    assert_eq!(true, guess_letter(word, guess));
}

#[test]
fn can_guess_letter_incorrect() {
    let word = "APPLE";
    let guess = String::from("B"); 
    assert_eq!(false, guess_letter(word, guess));
}

// #[test]
// fn can_input() {
//     let input = get_input();
//     assert_eq!(1, input.len());
// }

#[test]
fn can_update_letters_correct() {
    let mut user_letters: HashSet<String> = HashSet::new();
    let chosen_word = String::from("HIPPO");
    let guess = String::from("P");
    assert_eq!(update_user_letters(chosen_word.as_str(), guess, &mut user_letters), true);
}

#[test]
fn can_update_letters_incorrect() {
    let mut user_letters: HashSet<String> = HashSet::new();
    let chosen_word = String::from("HIPPO");
    let guess = String::from("A");
    assert_eq!(update_user_letters(chosen_word.as_str(), guess, &mut user_letters), false);
}

#[test]
fn can_print_word() {
    let mut user_letters: HashSet<String> = HashSet::new();
    let chosen_word = String::from("HIPPO");
    let guess = String::from("H");
    update_user_letters(chosen_word.as_str(), guess, &mut user_letters);
    assert_eq!(print_word(chosen_word.as_str(), &user_letters), "H _ _ _ _ ");
}
