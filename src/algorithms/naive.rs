use crate::{Guess, Guesser};

pub struct Naive;

impl Naive {
	pub fn new() -> Naive {
		Naive
	}
}

impl crate::Guesser for Naive {
	fn guess(&mut self, _history: &[crate::Guess]) -> String {
		String::from("guess")
	}
}
