

use super::char_index::char_to_index;
use super::errors::TrieError;
use super::letters::{Letter, Letters};

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::str::Chars;

use deepsize::DeepSizeOf;
use serde::{Serialize, Deserialize};

type ChildNode = Option<Rc<RefCell<Node>>>;


#[derive(Debug, Default, Serialize, Deserialize, DeepSizeOf)]
pub struct Node {
  children: [ChildNode; 26],
  eow: Cell<bool>, // true if node is end of word
}

impl Node {
  pub fn new() -> Node {
    Node { children: Default::default(), eow: Cell::new(false) }
  }

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