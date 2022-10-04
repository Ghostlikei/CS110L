// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;

use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;


const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn to_string(vec: &Vec<char>) -> String{
    let mut result = String::new();
    for each in vec.iter(){
        result.push(*each);
    }
    result
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut chance: u32 = NUM_INCORRECT_GUESSES;
    let mut guess_ans: Vec<char> = Vec::new();
    let mut guessed_letter:Vec<char> = Vec::new();
    let mut unrevealed_num = secret_word_chars.len();
    for _n in 0..(secret_word_chars.len()) {
        guess_ans.push('-');
    }
    loop{
        if chance == 0 {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        else {
            // Print Status first
            println!("The word so far is {}", to_string(&guess_ans));
            println!("You have guessed the following letters: {}", to_string(&guessed_letter));
            println!("You have {} guesses left", chance);
            print!("Please guess a letter :");
            io::stdout().flush().expect("Error flushing stdout.");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Error reading line.");
            let letters: Vec<char> = guess.chars().collect();
            let letter = letters[0];
            guessed_letter.push(letter);
            // Implement the find_all function
            let mut flag: i32 = 0;  // Use the flag to mark whether it is a valid guess
            let mut index: usize = 0; // Use the index to reveal the right letter position
            for each in secret_word_chars.iter(){
                if *each == letter{
                    flag = 1;
                    unrevealed_num -= 1;
                    guess_ans[index] = letter;
                }
                index += 1;
            }

            if flag == 0{
                chance -= 1;
                println!("Sorry, that letter is not in the word");
            }

            if unrevealed_num == 0 {
                println!("\nCongratulations you guessed the secret word: {}!", to_string(&guess_ans));
                break;
            }
            println!("");
        }
    }
}
