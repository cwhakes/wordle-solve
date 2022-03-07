use std::collections::BTreeMap;

use crate::{Guess, Guesser, DICTIONARY};

pub struct Naive {
	remaining: BTreeMap<&'static str, u32>,
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
	count: u32,
	goodness: f64,
}

impl Guesser for Naive {
	fn guess(&mut self, history: &[Guess]) -> String {
		if let Some(last) = history.last() {
			self.remaining.retain(|w, _| last.matches(w))
		}

		let mut best: Option<Candidate> = None;
		for (&word, &count) in &self.remaining {
			let goodness = goodness(word, count, history);
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

		best.expect("Should always find a word").word.to_string()
	}
}

fn goodness(_guess: &str, count: u32, _history: &[crate::Guess]) -> f64 {
	count as f64
}
