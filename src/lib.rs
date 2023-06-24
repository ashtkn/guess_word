const WORDS: &str = include_str!("words.txt");

use std::collections::HashSet;

use rand::seq::SliceRandom;

pub struct Dictionary {
    words: HashSet<&'static str>,
}

impl Dictionary {
    pub fn new() -> Self {
        let words: HashSet<&str> = WORDS.split("\n").collect();
        Self { words }
    }
    pub fn get_random_words(&self) -> String {
        Vec::from_iter(self.words.iter())
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

pub struct Game {
    answer: String,
    dictonary: Dictionary,
}

impl Default for Game {
    fn default() -> Self {
        let dict = Dictionary::new();
        Game {
            answer: dict.get_random_words(),
            dictonary: dict,
        }
    }
}

impl Game {
    pub fn get_answer(&self) -> String {
        self.answer.to_string()
    }
    pub fn in_dictionary(&self, word: &str) -> bool {
        self.dictonary.words.get(word).is_some()
    }
}
