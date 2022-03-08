use std::collections::BTreeMap;

use crate::{Guess, Guesser, DICTIONARY, Correctness};

pub struct Naive {
	remaining: BTreeMap<&'static str, u64>,
}

impl Naive {
	pub fn new() -> Naive {
		Naive {
			remaining: DICTIONARY
				.lines()
				.map(|line| {
					let (w, n) = line.split_once(' ').expect("expecting format `word ###`");
					let n = n.parse().expect("expecting a number");
					(w, n)
				})
				.collect(),
		}
	}
}

#[derive(Debug, Clone)]
struct Candidate {
	word: &'static str,
	count: u64,
	goodness: f64,
}

impl Guesser for Naive {
	fn guess(&mut self, history: &[Guess]) -> &'static str {
		if let Some(last) = history.last() {
			self.remaining.retain(|w, _| last.matches(w))
		} else {
			return "tares";
		}

		let total: u64 = self.remaining.values().sum();
		let mut best: Option<Candidate> = None;
		for (&word, &count) in &self.remaining {
			let total_goodness: f64 = Correctness::permutations().map(|mask| {
				let words_left: u64 = self.remaining.iter()
					.filter(|(w, _)| {
						Guess {
							word,
							mask,
						}.matches(w)
					})
					.map(|(_, c)| *c)
					.sum();
				let p_pattern = words_left as f64 / total as f64;
				if p_pattern == 0.0 {
					0.0
				} else {
					p_pattern * -(p_pattern.log2())
				}
			}).sum();

			let goodness = total_goodness / (3.0f64.powi(5));
			if let &mut Some(ref mut best) = &mut best {
				if goodness > best.goodness {
					*best = Candidate {
						word,
						count,
						goodness,
					};
				}
			} else {
				best = Some(Candidate {
					word,
					count,
					goodness,
				});
			}
		}
		if best.is_none() {
			eprintln!("{:?}", history);
		}

		best.expect("Should always find a word").word
	}
}
