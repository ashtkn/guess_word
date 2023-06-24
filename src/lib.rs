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
