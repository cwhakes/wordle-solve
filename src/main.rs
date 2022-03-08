use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

fn main() {
	let w = Wordle::new();
	for answer in ANSWERS.lines() {
		println!("{}:", answer);
		let guesser = algorithms::Naive::new();
		let count = w.play(answer, guesser);
		println!(" {:?}", count);
	}
}
