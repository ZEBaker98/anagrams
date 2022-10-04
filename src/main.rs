
use std::env;
use std::path::Path;

mod trie;
use trie::letters::Letters;

mod test;
use test::{time_trie_creation, time_anagram_solve};

enum InputError {
    InvalidInput(String),
    MissingInput,
}

fn print_examples() {
    println!("Examples:");
    println!("\tcargo run --release -- alphabetize");
    println!("\tcargo run --release -- blue???");
}

fn get_input() -> Result<String, InputError> {
    let mut args = env::args();
    match args.nth(1) {
        Some(input) => {
            if Letters::from_string(input.clone()).is_ok() {
                Ok(input)
            } else {
                Err(InputError::InvalidInput(input))
            }
        },
        None => Err(InputError::MissingInput),
    }
}

fn main() {

    match get_input() {
        Ok(input) => {
            let path = Path::new("./words_alpha.txt");

            let t = time_trie_creation(path);

            let mut words = time_anagram_solve(&t, input);
            words.sort_by_key(|x| x.len());

            for word in words {
                println!("{}", word);
            }
        },
        Err(InputError::MissingInput) => {
            println!("Argument is missing! Anagrams input can contain alphabetic characters and ? as wildcard characters.");
            print_examples();
        },
        Err(InputError::InvalidInput(input)) => {
            println!("Argument '{}' is invalid! Input can contain alphabetic characters and ? as wildcard characters.", input);
            print_examples();
        }
    }
}
