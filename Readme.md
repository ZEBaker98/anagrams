# Anagram solver

This program was written as a way to learn rust. This is my first project in rust.

This program can process a txt file of words into a prefix tree that can then be searched rapidly for anagrams of a given set of letters.

words_alpha.txt was retrieved from https://github.com/dwyl/english-words

# Usage

Anagrams can be run through cargo. It accepts one argument as input to search for anagrams for.

## Examples

`cargo run -- "incandescent"` will find all words that can be made from the letters in "incandescent".

`cargo run -- "egg?"` will find all words that can be made from "egg" plus one additional wild character.

Using the release tag will build the release version. This will speed up execution by a great deal.
`cargo run --release -- "faster"`