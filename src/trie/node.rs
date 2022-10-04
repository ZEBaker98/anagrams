

use super::char_index::char_to_index;
use super::errors::TrieError;
use super::letters::{Letter, Letters};

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::str::Chars;

use deepsize::DeepSizeOf;

type ChildNode = Option<Rc<RefCell<Node>>>;

// a node of the trie
#[derive(Debug, Default, DeepSizeOf)]
pub struct Node {
  children: [ChildNode; 26], // contains up to 26 children, one for each letter
  eow: Cell<bool>, // true if node is end of word
}

impl Node {
  pub fn new() -> Node {
    Node { children: Default::default(), eow: Cell::new(false) }
  }

  // get a child node by char index
  pub fn get(&self, i: char) -> Result<Option<Rc<RefCell<Node>>>, TrieError> {
    match char_to_index(i) {
      Ok(index) => {
        match &self.children[index] {
          Some(child) => {
            Ok(Some(child.clone()))
          }
          None => Ok(None),
        }
      },
      Err(e) => Err(e),
    }
  }

  // get a child node by a char index or create it if it doesn't exist
  pub fn get_or_create(&mut self, i:char) -> Result<Rc<RefCell<Node>>, TrieError> {
    match char_to_index(i) {
      Ok(index) => {
        match &self.children[index] {
          Some(child) => {
            Ok(child.clone())
          }
          None => {
            let child = Rc::new(RefCell::new(Node::new()));
            self.children[index] = Some(child.clone());
            Ok(child)
          }
        }
      },
      Err(e) => Err(e),
    }
  }

  // recursively ensure word is in trie
  pub fn insert(&mut self, mut word: Chars) -> Result<(), TrieError> {
    match word.next() {
      Some(letter) => {
        match self.get_or_create(letter) {
          Ok(node) => {
            node.as_ref().borrow_mut().insert(word)?;
            Ok(())
          },
          Err(e) => Err(e),
        }
      },
      None => {
        self.eow.set(true);
        Ok(())
      },
    }
  }

  // recursively determine if word is in trie
  pub fn find(&self, mut word: Chars) -> Result<bool, TrieError> {
    match word.next() {
      Some(letter) => {
        match self.get(letter) {
          Ok(Some(node)) => {
            node.as_ref().borrow().find(word)
          },
          Ok(None) => {
            Ok(false)
          },
          Err(e) => Err(e),
        }
      }
      None => {
        Ok(self.eow.get())
      }
    }
  }

  // recursively generate all anagrams from a set of letters
  pub fn anagrams(&self, letters: Letters, prefix: String) -> Result<Vec<String>, TrieError> {
    let mut words: Vec<String> = Vec::new();
    if self.eow.get() {
      words.push(prefix.clone());
    }
    for Letter(letter, remaining) in letters {
      match self.get(letter) {
        Ok(Some(child)) => {
          let mut next_prefix = prefix.clone();
          next_prefix.push(letter);
          match child.as_ref().borrow().anagrams(remaining, next_prefix) {
            Ok(mut w) => {
              words.append(&mut w);
            },
            Err(e) => return Err(e),
          }
          ;
        },
        Ok(None) => {},
        Err(e) => return Err(e),
      }
    }
    Ok(words)
  }

  // recursively count child nodes
  pub fn node_count(&self) -> i32 {
    let mut nodes = 1;
    for child in &self.children {
      match child {
        Some(node) => {
          nodes += node.as_ref().borrow().node_count();
        },
        None => {},
      }
    }
    nodes
  }
}