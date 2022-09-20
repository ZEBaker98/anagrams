use super::char_index::{char_to_index, index_to_char};
use super::errors::TrieError;


pub struct Letter(pub char, pub Letters);

#[derive(Debug, Clone)]
pub struct Letters {
  array: [i32; 26],
  wild: i32,
}

impl Letters {
  pub fn from_string(s: String) -> Result<Letters, TrieError> {
    let mut letters = [0; 26];
    let mut wild: i32 = 0;
    for letter in s.chars() {
      if letter != '?' {
        match char_to_index(letter) {
          Ok(index) => {
            letters[index] += 1;
          },
          Err(e) => {
            return Err(e)
          }
        }
      } else {
        wild += 1;
      }
    }
    Ok(Letters{ array: letters, wild: wild })
  }

  #[allow(dead_code)]
  pub fn contains(&self, other: &Letters) -> bool {
    for i in 0..26 {
      if other.array[i] > self.array[i] {
        return false
      }
    }
    return true;
  }
}

impl IntoIterator for Letters {
  type Item = Letter;
  type IntoIter = LettersIter;

  fn into_iter(self) -> Self::IntoIter {
      LettersIter::new(self)
  }
}

pub struct LettersIter {
  index: usize,
  letters: Letters,
}

impl LettersIter {
  fn new(letters: Letters) -> LettersIter {
    LettersIter { index: 0, letters: letters }
  }
}

impl Iterator for LettersIter {
  type Item = Letter;

  fn next(&mut self) -> Option<Self::Item> {
    while self.index < 26 {
      let c = index_to_char(self.index).expect("Invalid index to char conversion");
      if self.letters.array[self.index] > 0 {
        let mut remaining = self.letters.clone();
        remaining.array[self.index] -= 1;
        self.index += 1;
        return Some(Letter(c, remaining));
      } else if self.letters.wild > 0 {
        let mut remaining = self.letters.clone();
        remaining.wild -= 1;
        self.index += 1;
        return Some(Letter(c, remaining));
      }
      self.index += 1;
    }
    None
  }
}