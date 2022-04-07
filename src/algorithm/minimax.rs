use std::collections::BTreeMap;

use crate::{Correctness, Guess, Guesser, DICTIONARY};

#[derive(Clone)]
pub struct Minimax {
	dictionary: BTreeMap<&'static str, f64>,
	remaining: BTreeMap<&'static str, f64>,
}

impl Default for Minimax {
	fn default() -> Self {
		let dictionary: BTreeMap<&'static str, f64> = DICTIONARY
			.lines()
			.map(|line| {
				let (w, n) = line.split_once(' ').expect("expecting format `word ###`");
				let f: f64 = n.parse().expect("expecting a number");
				// Apply sigmoid
				let f = f / (f + 10000.0);
				(w, f)
			})
			.collect();

		Self {
			dictionary: dictionary.clone(),
			remaining: dictionary,
		}
	}
}

#[derive(Debug, Clone)]
struct Candidate {
	word: &'static str,
	goodness: f64,
}

impl Guesser for Minimax {
	fn guess(&mut self, history: &[Guess]) -> &'static str {
		if let Some(last) = history.last() {
			self.remaining.retain(|w, _| last.matches(w));
		} else {
			return "lasso";
		}

		let total: f64 = self.remaining.values().sum();
		let mut best: Option<Candidate> = None;
		for word in self.remaining.keys() {
			let goodness: f64 = Correctness::permutations()
				.map(|mask| {
					let words_left: f64 = self
						.remaining
						.iter()
						.filter(|(w, _)| Guess::from_parts(word, mask).matches(w))
						.map(|(_, c)| *c)
						.sum();
					let p_pattern = words_left / total;
					if p_pattern == 0.0 {
						0.0
					} else {
						p_pattern * -(p_pattern.log2())
					}
				})
				.filter(|&x| x > f64::EPSILON)
				.min_by(|x, y| x.partial_cmp(y).unwrap())
				.unwrap_or(f64::INFINITY);

			if let &mut Some(ref mut best) = &mut best {
				if goodness > best.goodness {
					*best = Candidate { word, goodness };
				}
			} else {
				best = Some(Candidate { word, goodness });
			}
		}
		if best.is_none() {
			eprintln!("{:?}", history);
		}

		best.expect("Should always find a word").word
	}

	fn reset(&mut self) {
		self.remaining = self.dictionary.clone();
	}
}
