#![allow(dead_code)]

use super::errors::TrieError;
use super::letters::Letters;
use super::node::Node;

use std::cell::RefCell;
use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::fs::File;

use deepsize::DeepSizeOf;

// usable trie struct
#[derive(Debug, DeepSizeOf)]
pub struct Trie {
  root: RefCell<Node>,
}

impl Trie {
  pub fn new() -> Trie {
    Trie { root: RefCell::new(Node::new()) }
  }

  // create trie from list of words
  pub fn from_word_list(path: &Path) -> io::Result<Trie> {
    let t = Trie::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
      match line {
        Ok(string) => {
          let i = t.insert(string.clone());
          if i.is_err() {
            println!("Encountered error inserting word {}", string);
          }
        },
        Err(e) => {
          println!("Encountered error {:?} while reading from file, continuing", e);
        }
      }
    }
    Ok(t)
  }

  // insert word into trie
  pub fn insert(&self, word: String) -> Result<(), TrieError> {
    self.root.borrow_mut().insert(word.chars())?;
    Ok(())
  }

  // determine if word is in trie
  pub fn find(&self, word: String) -> Result<bool, TrieError> {
    self.root.borrow().find(word.chars())
  }

  // find all anagrams for letters in trie
  pub fn anagrams(&self, letters: String) -> Result<Vec<String>, TrieError> {
    let letters = Letters::from_string(letters)?;
    let words = self.root.borrow().anagrams(letters, String::from(""))?;
    Ok(words)
  }

  // count all nodes in trie
  pub fn node_count(&self) -> i32 {
    self.root.borrow().node_count()
  }
}

