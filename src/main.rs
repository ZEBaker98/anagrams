
use std::path::Path;

mod trie;

mod test;
use test::{time_trie_creation, time_anagram_solve};

fn main() {
    let path = Path::new("./words_alpha.txt");

    let t = time_trie_creation(path);

    let mut words = time_anagram_solve(&t, String::from("incandescent"));
    words.sort_by_key(|x| x.len());

    for word in words {
        println!("{}", word);
    }
}
