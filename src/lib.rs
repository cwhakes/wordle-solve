pub mod algorithm {
	mod minimax;
	pub use minimax::Minimax;

	mod naive;
	pub use naive::Naive;

	mod with_vec;
	pub use with_vec::WithVec;

	mod with_word;
	pub use with_word::WithWord;
}

use std::collections::BTreeSet;
use std::fmt;

use itertools::{iproduct, Itertools};

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
	dictionary: BTreeSet<Word>,
	history: Vec<Guess>,
}

impl Default for Wordle {
	fn default() -> Self {
		Self {
			dictionary: DICTIONARY
				.lines()
				.filter_map(|w| w.split_once(' ').and_then(|w| Word::new(w.0)))
				.collect(),
			history: Vec::new(),
		}
	}
}

impl Wordle {
	pub fn play<G: Guesser>(&mut self, answer: &'static str, mut guesser: G) -> Option<u8> {
		let answer: &[u8] = answer.as_ref();

		for n in 1..=6 {
			let guess = guesser.guess(&self.history);
			debug_assert!(self.dictionary.contains(&Word::new(&guess).unwrap()));
			if guess.as_ref() == answer {
				return Some(n);
			}
			let correctness = Correctness::check(answer, &guess);
			let guess = Guess::from_parts(&guess, correctness);
			println!("Guessed: {}", guess);
			self.history.push(guess);
		}
		None
	}

	pub fn recommend<G: Guesser>(&self, guesser: &mut G) -> String {
		String::from_utf8_lossy(guesser.guess(&self.history).as_ref()).to_string()
	}

	pub fn validate_guess(&self, guess: &str) -> bool {
		self.dictionary.contains(&Word::new(guess).unwrap())
	}

	pub fn guess(&mut self, guess: Guess) {
		self.history.push(guess);
	}

	pub fn reset(&mut self) {
		self.history.clear()
	}
}

#[derive(Debug)]
pub struct Guess {
	word: Word,
	mask: [Correctness; 5],
}

impl Guess {
	pub fn new(word: impl AsRef<[u8]>, mask: &str) -> Option<Self> {
		let mask = Correctness::new(mask)?;
		Some(Self::from_parts(word, mask))
	}

	pub fn is_correct(&self) -> bool {
		self.mask == Correctness::CORRRECT
	}

	fn from_parts(word: impl AsRef<[u8]>, mask: [Correctness; 5]) -> Self {
		let word: &[u8] = word.as_ref();
		assert_eq!(5, word.len());
		Self {
			word: Word(<[u8; 5]>::try_from(word).unwrap()),
			mask,
		}
	}

	fn matches(&self, word: impl AsRef<[u8]>) -> bool {
		let word = word.as_ref();

		let mut used = [false; 5];
		for i in 0..5 {
			let (g, w, m) = (self.word.0[i], word[i], self.mask[i]);
			match (m == Correctness::Correct, g == w) {
				(true, true) => used[i] = true,
				(false, false) => { /* do nothing */ },
				_ => return false,
			}
		}
		'outer: for (g, m) in self.word.0.iter().zip(self.mask) {
			match m {
				Correctness::Misplaced => {
					// inner loop
					for (w, u) in word.iter().zip(&mut used) {
						if !*u && w == g {
							*u = true;
							// Only contiue if something is found in inner loop
							continue 'outer;
						}
					}
					return false;
				},
				Correctness::Wrong => {
					for (w, u) in word.iter().zip(&mut used) {
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
		for c in iter {
			write!(f, " {}", c)?;
		}
		write!(f, "]")
	}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word([u8; 5]);

impl Word {
	fn new(word: impl AsRef<[u8]>) -> Option<Self> {
		word.as_ref().try_into().ok().map(Self)
	}
}

impl AsRef<[u8]> for Word {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl fmt::Debug for Word {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\"{}\"", String::from_utf8_lossy(&self.0))
	}
}

impl fmt::Display for Word {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", String::from_utf8_lossy(&self.0))
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Correctness {
	Correct,
	Misplaced,
	Wrong,
}

impl Correctness {
	const CORRRECT: [Self; 5] = [Self::Correct; 5];

	fn new(s: &str) -> Option<[Self; 5]> {
		s.chars()
			.filter_map(|c| match c {
				'c' | 'C' => Some(Self::Correct),
				'm' | 'M' => Some(Self::Misplaced),
				'w' | 'W' => Some(Self::Wrong),
				_ => None,
			})
			.collect_tuple()
			.map(|(a, b, c, d, e)| [a, b, c, d, e])
	}

	fn check(answer: impl AsRef<[u8]>, guess: impl AsRef<[u8]>) -> [Self; 5] {
		let mut correctness = [Self::Wrong; 5];
		let answer = <[u8; 5]>::try_from(answer.as_ref()).unwrap();
		let guess = <[u8; 5]>::try_from(guess.as_ref()).unwrap();

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

	fn permutations() -> impl Iterator<Item = [Self; 5]> {
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
	type GuessFormat: AsRef<[u8]> + 'static;

	fn guess(&mut self, history: &[Guess]) -> Self::GuessFormat;
	fn reset(&mut self);
}

impl<'a, G: Guesser + ?Sized> Guesser for &'a mut G {
	type GuessFormat = G::GuessFormat;

	fn guess(&mut self, history: &[Guess]) -> Self::GuessFormat {
		(&mut **self).guess(history)
	}

	fn reset(&mut self) {
		(&mut **self).reset();
	}
}

#[cfg(test)]
macro_rules! guesser {
	(|$history: ident| $impl:block) => {{
		struct G;
		impl $crate::Guesser for G {
			type GuessFormat = &'static str;

			fn guess(&mut self, $history: &[Guess]) -> &'static str {
				$impl
			}
			fn reset(&mut self) {}
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
			let mut w = Wordle::default();
			let guesser = guesser!(|_history| { "right" });
			assert_eq!(w.play("right", guesser), Some(1));
		}
		#[test]
		fn play32() {
			let mut w = Wordle::default();
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
				assert!(Guess::from_parts($prev, mask![$($mask )+]).matches($next))
			};
			($prev:literal [$($mask:tt)+] disallows $next:literal) => {
				assert!(!Guess::from_parts($prev, mask![$($mask )+]).matches($next))
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

		#[test]
		fn test_bumph() {
			check!("bumph" [W C C C C] allows "humph");
		}
	}
}
