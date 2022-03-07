pub mod algorithms {
	mod naive;
	pub use naive::Naive;
}

pub fn play<G: Guesser>(answer: &'static str, guesser: G) -> Option<u8> {
	None
}

pub struct Guess {
	word: String,
	mask: [Correctness; 5],
}

pub enum Correctness {
	Correct,
	Misplaced,
	Wrong,
}

pub trait Guesser {
	fn guess(&mut self, history: &[Guess]) -> String {
		String::default()
	}
}
