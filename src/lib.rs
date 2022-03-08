use std::collections::BTreeSet;

pub mod algorithms {
	mod naive;
	pub use naive::Naive;
}

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
	dictionary: BTreeSet<&'static str>,
}

impl Wordle {
	pub fn new() -> Self {
		Self {
			dictionary: DICTIONARY
				.lines()
				.filter_map(|w| w.split_once(' ').map(|w| w.0))
				.collect(),
		}
	}

	pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<u8> {
		let mut history = Vec::new();
		for n in 1..=32 {
			let guess = guesser.guess(&history);
			debug_assert!(self.dictionary.contains(&*guess));
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
}

#[derive(Debug)]
pub struct Guess {
	word: String,
	mask: [Correctness; 5],
}

impl Guess {
	fn matches(&self, word: &str) -> bool {
		let mut used = [false; 5];
		for (i, ((g, w), m)) in self
			.word
			.chars()
			.zip(word.chars())
			.zip(self.mask)
			.enumerate()
		{
			if m == Correctness::Correct {
				if g != w {
					return false;
				} else {
					used[i] = true;
				}
			} else {
				if g == w {
					return false;
				}
			}
		}
		'outer: for ((g, m), u) in self.word.chars().zip(self.mask).zip(&mut used) {
			if m == Correctness::Misplaced {
				// inner loop
				for w in word.chars() {
					if !*u && w == g {
						*u = true;
						// Only contiue if something is found in inner loop
						continue 'outer;
					}
				}
				return false;
			}
		}

		true
	}
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
	fn guess(&mut self, history: &[Guess]) -> String;
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
macro_rules! guesser {
	(|$history: ident| $impl:block) => {{
		struct G;
		impl $crate::Guesser for G {
			fn guess(&mut self, $history: &[Guess]) -> String {
				$impl
			}
		}
		G
	}};
}

#[cfg(test)]
macro_rules! mask {
	(C) => {$crate::Correctness::Correct};
	(M) => {$crate::Correctness::Misplaced};
	(W) => {$crate::Correctness::Wrong};
	($($c:tt)+) => {[
		$(mask!($c)),+
	]}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod game {
		use super::{Guess, Wordle};
		#[test]
		fn play1() {
			let w = Wordle::new();
			let guesser = guesser!(|_history| { String::from("right") });
			assert_eq!(w.play("right", guesser), Some(1));
		}
		#[test]
		fn play32() {
			let w = Wordle::new();
			let guesser = guesser!(|_history| { String::from("wrong") });
			assert_eq!(w.play("right", guesser), None);
		}
	}
	mod correctness {
		use super::Correctness;

		#[test]
		fn test_correctness() {
			assert_eq!(Correctness::check("aaabb", "abbab"), mask![C M W M C])
		}
	}

	mod guess {
		use super::Guess;

		#[test]
		fn test_similar_word() {
			assert!(Guess {
				word: "crate".to_string(),
				mask: mask![W C C C C],
			}
			.matches("grate"))
		}

		#[test]
		fn test_disimilar_word() {
			assert!(Guess {
				word: "sugar".to_string(),
				mask: mask![W W W M M],
			}
			.matches("hoard"))
		}

		#[test]
		fn test_mania1() {
			assert!(Guess {
				word: "which".to_string(),
				mask: mask![W W M W W],
			}
			.matches("mania"))
		}
		#[test]
		fn test_mania2() {
			assert!(Guess {
				word: "first".to_string(),
				mask: mask![W M W W W],
			}
			.matches("mania"))
		}
		#[test]
		fn test_mania3() {
			assert!(Guess {
				word: "again".to_string(),
				mask: mask![M W M C M],
			}
			.matches("mania"))
		}
	}
}
