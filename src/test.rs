#![allow(dead_code)]

use std::path::Path;
use std::time::Instant;
use deepsize::DeepSizeOf;

use super::trie::Trie;


pub fn time_trie_creation(path: &Path) -> Trie {
  print!("Building trie from {:?}...", path);
  let now = Instant::now();
  let t: Trie = Trie::from_word_list(path).expect("Error creating trie from file");
  let elapsed = now.elapsed();
  println!("\tFinished after {:?}", elapsed);
  t
}

pub fn time_anagram_solve(t: &Trie, letters: String) -> Vec<String> {
  print!("Solving for anagrams of '{}'", letters);
  let now = Instant::now();
  let words = t.anagrams(letters).expect("Failed to solve for anagrams");
  let elapsed = now.elapsed();
  println!("\tFinished after {:?}", elapsed);
  words
}


pub fn check_word(t: &Trie, word: String) {
    match t.find(word.clone()) {
        Ok(found) => println!("Checking for {} in trie: {}", word, found),
        Err(_) => println!("Error ocurred while checking for word"),
    }
    ;
}

pub fn trie_size(t: &Trie) {
    let empty_size = Trie::new().deep_size_of();
    println!("Size of empty Trie: {} bytes", empty_size);

    let nodes = t.node_count();
    println!("Nodes in trie: {}", nodes);

    println!("Predicted size of filled Trie: {} bytes", empty_size * nodes as usize);
    println!("Actual size of filled Trie: {} kb", t.deep_size_of() as f32 / 1000.0);
}