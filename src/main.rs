const WORD_LIST: [&str; 5] = ["apple", "building", "bob", "notepad", "keyboard"];

use rand::Rng;
fn choose_word(word_list: [&str; 5]) -> String {
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

fn main() {
    println!("Hello, world!");
    let input = get_input();
    println!("You entered: {}", input);
}

#[test]
fn can_choose_word() {
    let chosen_word = choose_word(WORD_LIST); 
    assert_eq!(true, WORD_LIST.contains(&chosen_word.as_str()));
}

#[test]
fn can_input() {
    let input = get_input();
    println!("a");
    assert_eq!(1, input.len());
}