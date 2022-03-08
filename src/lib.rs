use std::collections::BTreeSet;
use std::fmt;

use itertools::iproduct;

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
			let guess = Guess {
				word: guess,
				mask: correctness,
			};
			println!("Guessed: {}", guess);
			history.push(guess);
		}
		None
	}
}

#[derive(Debug)]
pub struct Guess {
	word: &'static str,
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
		'outer: for (g, m) in self.word.chars().zip(self.mask) {
			match m {
				Correctness::Misplaced => {
					// inner loop
					for (w, u) in word.chars().zip(&mut used) {
						if !*u && w == g {
							*u = true;
							// Only contiue if something is found in inner loop
							continue 'outer;
						}
					}
					return false;
				},
				Correctness::Wrong => {
					for (w, u) in word.chars().zip(&mut used) {
						if !*u && w == g {
							return false;
						}
					}
				},
				_ => {},
			}
		}

		true
	}
}

impl fmt::Display for Guess {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} [", self.word)?;
		let mut iter = self.mask.iter();
		if let Some(c) = iter.next() {
			write!(f, "{}", c)?;
		}
		while let Some(c) = iter.next() {
			write!(f, " {}", c)?;
		}
		write!(f, "]")
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

	pub fn permutations() -> impl Iterator<Item = [Self; 5]> {
		let x = [Self::Correct, Self::Misplaced, Self::Wrong];
		iproduct!(x, x, x, x, x).map(|(a, b, c, d, e)| [a, b, c, d, e])
	}
}

impl fmt::Display for Correctness {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Correct => write!(f, "C"),
			Self::Misplaced => write!(f, "M"),
			Self::Wrong => write!(f, "W"),
		}
	}
}

pub trait Guesser {
	fn guess(&mut self, history: &[Guess]) -> &'static str;
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
			fn guess(&mut self, $history: &[Guess]) -> &'static str {
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
			let guesser = guesser!(|_history| { "right" });
			assert_eq!(w.play("right", guesser), Some(1));
		}
		#[test]
		fn play32() {
			let w = Wordle::new();
			let guesser = guesser!(|_history| { "wrong" });
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

		macro_rules! check {
			($prev:literal [$($mask:tt)+] allows $next:literal) => {
				assert!(Guess {
					word: $prev,
					mask: mask![$($mask )+],
				}
				.matches($next))
			};
			($prev:literal [$($mask:tt)+] disallows $next:literal) => {
				assert!(!Guess {
					word: $prev,
					mask: mask![$($mask )+],
				}
				.matches($next))
			};
		}

		#[test]
		fn test_wrong_word() {
			check!("right" [C C C C C] disallows "wrong");
		}

		#[test]
		fn test_similar_word() {
			check!("crate" [W C C C C] allows "grate");
		}

		#[test]
		fn test_disimilar_word() {
			check!("sugar" [W W W M M] allows "hoard");
		}

		#[test]
		fn test_sheep() {
			check!("baaaa" [W C M W W] allows "aaccc")
		}

		#[test]
		fn test_mania() {
			check!("which" [W W M W W] allows "mania");
			check!("first" [W M W W W] allows "mania");
			check!("again" [M W M C M] allows "mania");
		}

		#[test]
		fn test_tares() {
			check!("tares" [W W W W W] disallows "areae");
		}
	}
}
