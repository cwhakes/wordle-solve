use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

fn main() {
	for answer in ANSWERS.lines() {
		let guesser = algorithms::Naive::new();
		play(answer, guesser);
	}
}
