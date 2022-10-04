
use super::errors::TrieError;

const LUT: [char; 26] = [
  'a', 'b', 'c', 'd', 'e', 
  'f', 'g', 'h', 'i', 'j', 
  'k', 'l', 'm', 'n', 'o',
  'p', 'q', 'r', 's', 't', 
  'u', 'v', 'w', 'x', 'y', 
  'z',
];

// Convert char to an index for an array
pub fn char_to_index(i: char) -> Result<usize, TrieError> {
  if i.is_ascii_alphabetic() {
    Ok(i.to_ascii_lowercase() as usize - 'a' as usize)
  } else {
    Err(TrieError::NonAlphabeticIndexError)
  }
}

// Convert integer index back to char
pub fn index_to_char(i: usize) -> Result<char, TrieError> {
  if i < 26 {
    Ok(LUT[i])
  } else {
    Err(TrieError::NonAlphabeticIndexError)
  }

}