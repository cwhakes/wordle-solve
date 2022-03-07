use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

fn main() {
	let w = Wordle::new();
	for answer in ANSWERS.lines() {
		let guesser = algorithms::Naive::new();
		w.play(answer, guesser);
	}
}
