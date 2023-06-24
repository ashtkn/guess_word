use std::io;

use guess_word::{Game, GameStatus, GuessResult, HitAccuracy};

fn main() {
    let mut game = Game::default();
    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let (state, result) = game.guess(guess.trim());

        match state {
            GameStatus::Won => {
                println!("You win!");
                break;
            }
            GameStatus::Lost => {
                println!("You lost...(answer: {}", game.get_answer().unwrap());
                break;
            }
            GameStatus::InProgress => match result {
                GuessResult::DuplicateGuess => {
                    println!("Warning: Duplicate guess.");
                }
                GuessResult::IncorrectLength => {
                    println!("Warning: Incorrect length.");
                }
                GuessResult::NotInDictionary => {
                    println!("Warning: Not in dictionary.");
                }
                GuessResult::Valid => {
                    let word_guess = game.guesses().last().unwrap();
                    let result: String = word_guess
                        .letters()
                        .iter()
                        .map(|l| match l.accuracy {
                            HitAccuracy::InRightPlace => "*".to_string(),
                            HitAccuracy::InWord => "!".to_string(),
                            HitAccuracy::NotInWord => " ".to_string(),
                        })
                        .collect();
                    println!("{}", result);
                }
                GuessResult::GameOver => {
                    println!("Warning: Game over.")
                }
            },
        }
        guess.clear();
    }
}
