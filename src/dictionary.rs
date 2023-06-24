const WORDS: &str = include_str!("words.txt");

use rand::seq::SliceRandom;
use std::collections::HashSet;

pub struct Dictionary {
    pub words: HashSet<&'static str>,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Dictionary {
    pub fn new() -> Self {
        let words: HashSet<&str> = WORDS.split('\n').collect();
        Self { words }
    }
    pub fn get_random_words(&self) -> String {
        Vec::from_iter(self.words.iter())
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}
