use std::collections::BinaryHeap;

pub mod algorithms {
	mod naive;
	pub use naive::Naive;
}

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<u16> {
	let mut history = Vec::new();
	for n in 1..=u16::MAX {
		let guess = guesser.guess(&history);
		if guess == answer {
			return Some(n);
		}
		let correctness = Correctness::check(answer, &guess);
		history.push(Guess {
			word: guess,
			mask: correctness,
		})
	}
	None
}

pub struct Guess {
	word: String,
	mask: [Correctness; 5],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Correctness {
	Correct,
	Misplaced,
	Wrong,
}

impl Correctness {
	fn check(answer: &str, guess: &str) -> [Self; 5] {
		let mut correctness = [Self::Wrong; 5];
		let answer = wordle_array(answer).unwrap();
		let guess = wordle_array(guess).unwrap();

		let mut used = [false; 5];

		for n in 0..5 {
			if answer[n] == guess[n] {
				correctness[n] = Self::Correct;
				used[n] = true;
			}
		}

		for n in 0..5 {
			if answer[n] == guess[n] {
				continue;
			}

			for m in 0..5 {
				if answer[m] == guess[n] && !used[m] {
					used[m] = true;
					correctness[n] = Self::Misplaced;
					break;
				}
			}
		}

		correctness
	}
}

pub trait Guesser {
	fn guess(&mut self, history: &[Guess]) -> String {
		String::default()
	}
}

fn wordle_array(word: &str) -> Option<[char; 5]> {
	if word.len() != 5 {
		return None;
	}

	let mut array = [char::default(); 5];
	for (n, c) in word.chars().enumerate() {
		array[n] = c
	}
	Some(array)
}

#[cfg(test)]
mod tests {
	use crate::Correctness;

	macro_rules! mask {
        (C) => {Correctness::Correct};
        (M) => {Correctness::Misplaced};
        (W) => {Correctness::Wrong};
        ($($c:tt)+) => {[
            $(mask!($c)),+
        ]}
    }

	#[test]
	fn test_correctness() {
		assert_eq!(Correctness::check("aaabb", "abbab"), mask![C M W M C])
	}
}
