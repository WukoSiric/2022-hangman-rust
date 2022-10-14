use std::{collections::HashSet}; //Storing correct letters
use rand::Rng; //Choosing random word

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
    return word_list[rng].to_string()
}

use std::io;
fn get_input() -> String {
    let mut guess = String::new();
    let mut guess_length = 0;

    while guess_length != 1 {
        guess = String::from("");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();
        guess_length = guess.len();
    }
    return guess
}

fn guess_letter(word: &str, letter: String) -> bool {
    return word.contains(letter.as_str())
}

fn update_correct_letters(word: &str, letter: String, correct_letters: &mut HashSet<String>) -> bool {
    let letter_exists: bool = guess_letter(word, letter.clone());
    if letter_exists {
        correct_letters.insert(letter);
        return true
    }
    return false
}

fn print_word(word: &str, correct_letters: &HashSet<String>) -> String {
    let mut output = String::new();
    for letter in word.chars() {
        if correct_letters.contains(&letter.to_string()) {
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
    println!("Hello, world!");
}

#[test]
fn can_choose_word() {
    let chosen_word = choose_word(WORD_LIST); 
    assert_eq!(true, WORD_LIST.contains(&chosen_word.as_str()));
}

#[test]
fn can_guess_letter_correct() {
    let word = "apple";
    let guess = String::from("a"); 
    assert_eq!(true, guess_letter(word, guess));
}

#[test]
fn can_guess_letter_incorrect() {
    let word = "apple";
    let guess = String::from("b"); 
    assert_eq!(false, guess_letter(word, guess));
}

#[test]
fn can_input() {
    let input = get_input();
    println!("a");
    assert_eq!(1, input.len());
}

#[test]
fn can_update_letters_correct() {
    let mut correct_letters: HashSet<String> = HashSet::new();
    let chosen_word = String::from("Hippo");
    let guess = String::from("p");
    assert_eq!(update_correct_letters(chosen_word.as_str(), guess, &mut correct_letters), true);
}

#[test]
fn can_update_letters_incorrect() {
    let mut correct_letters: HashSet<String> = HashSet::new();
    let chosen_word = String::from("Hippo");
    let guess = String::from("a");
    assert_eq!(update_correct_letters(chosen_word.as_str(), guess, &mut correct_letters), false);
}

#[test]
fn can_print_word() {
    let mut correct_letters: HashSet<String> = HashSet::new();
    let chosen_word = String::from("Hippo");
    let guess = String::from("H");
    update_correct_letters(chosen_word.as_str(), guess, &mut correct_letters);
    assert_eq!(print_word(chosen_word.as_str(), &correct_letters), "H _ _ _ _ ");
}
