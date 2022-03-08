use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

fn main() {
	let w = Wordle::new();
	for answer in ANSWERS.lines().take(25) {
		println!("{}:", answer);
		let guesser = algorithms::Naive::new();
		let count = w.play(answer, guesser);
		if let Some(count) = count {
			println!("Guessed {} in {} tries", answer, count);
		} else {
			println!("Failed to guess {}", answer);
		}

		
	}
}
