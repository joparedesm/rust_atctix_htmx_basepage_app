use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::pairs::Pair;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Word {
    pub words: Vec<Pair>
}

static WORDS: &str = include_str!("../assets/words.json");

// trait that defines the behavior of a Word
impl Word {
    // * Function to get all the words
    pub fn get_words() -> Word {
        // * Deserialize the JSON string into a Word struct by adding the id field
        let words: Word = serde_json::from_str(WORDS).unwrap();
        words
    }

    pub fn to_string(&self) -> String {
        self.words.iter()
            .map(Pair::to_string)
            .collect::<Vec<String>>()
            .join(",\n")
    }

    // * Function to get a word by its index
    pub fn get_random_words(&self) -> Pair {
        // * Get all the words
        let words = Word::get_words();
        let n_words = words.words.len();
        // * Get a random word pair
        let rng =  rand::thread_rng().gen_range(0..n_words);
        words.words[rng].clone()
    }

    // * Function to add a word
    pub fn add_word(word: Pair) -> Word {
        // * Get all the words
        let mut words = Word::get_words();
        // * Add the new word to the list of words
        words.words.push(word);
        words
    }

    // * Function to update a word
    pub fn update_word(index: usize, word: Pair) -> Word {
        // * Get all the words
        let mut words = Word::get_words();
        // * Update the word at the specified index
        words.words[index] = word;
        words
    }

    // * Function to delete a word
    pub fn delete_word(index: usize) -> Word {
        // * Get all the words
        let mut words = Word::get_words();
        // * Remove the word at the specified index
        words.words.remove(index);
        words
    }
}
