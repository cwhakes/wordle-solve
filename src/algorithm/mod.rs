mod naive;
pub use naive::Naive;

use crate::Guesser;

pub fn select(name: &str) -> Option<Box<dyn Guesser>> {
	match &*name.to_ascii_lowercase() {
		"naive" => Some(Box::new(Naive::new())),
		_ => None,
	}
}
