use clap::{ArgEnum, Parser, Subcommand};
use std::io::{stdin, stdout, Write};

use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

/// A program to solve wordles
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long, arg_enum, default_value_t = GuesserArg::WithVec)]
	guesser: GuesserArg,
	#[clap(subcommand)]
	command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
	/// Solve wordles from list in `answers.txt`
	List {
		#[clap(short, long)]
		max: Option<usize>,
	},
	/// Give ignorable suggestions for daily wordle
	Cheat,
}

impl Default for Command {
	fn default() -> Self {
		Self::Cheat
	}
}

#[derive(ArgEnum, Debug, Clone)]
enum GuesserArg {
	Minimax,
	Naive,
	WithVec,
}

fn main() {
	let args = Args::parse();

	dbg!(&args.guesser);
	match &args.command.unwrap_or_default() {
		Command::List { max } => match args.guesser {
			GuesserArg::Minimax => list(*max, algorithm::Minimax::default),
			GuesserArg::Naive => list(*max, algorithm::Naive::default),
			GuesserArg::WithVec => list(*max, algorithm::WithVec::default),
		},
		Command::Cheat => match args.guesser {
			GuesserArg::Minimax => cheat(algorithm::Minimax::default),
			GuesserArg::Naive => cheat(algorithm::Naive::default),
			GuesserArg::WithVec => cheat(algorithm::WithVec::default),
		},
	}
}

fn list<G>(max: Option<usize>, guesser: impl Fn() -> G)
where
	G: Guesser,
{
	let mut w = Wordle::default();
	let mut guesser = guesser();

	for answer in ANSWERS.lines().take(max.unwrap_or(usize::MAX)) {
		println!("{}:", answer);
		let count = w.play(answer, &mut guesser);
		if let Some(count) = count {
			println!("Guessed {} in {} tries", answer, count);
		} else {
			println!("Failed to guess {}", answer);
			panic!();
		}
		w.reset();
		guesser.reset();
	}
}

fn cheat<G>(guesser: impl Fn() -> G)
where
	G: Guesser,
{
	let mut w = Wordle::default();
	let mut guesser = guesser();
	for _ in 1..=6 {
		println!();
		let recommendation = w.recommend(&mut guesser);
		println!("Recommendation: {}", recommendation);

		let mut buf = String::new();
		let guess = loop {
			print!("      Guess: ");
			stdout().flush().unwrap();
			stdin().read_line(&mut buf).unwrap();
			let guess = buf.trim();

			if guess.is_empty() {
				println!("      Guess: {}", recommendation);
				break recommendation;
			} else if w.validate_guess(guess) {
				break guess;
			} else {
				println!("Guess not in dictionary");
				buf.clear();
			}
		};

		let mut buf = String::new();
		let guess = loop {
			print!("Correctness: ");
			stdout().flush().unwrap();
			stdin().read_line(&mut buf).unwrap();
			if let Some(guess) = Guess::new(guess, buf.trim()) {
				break guess;
			} else {
				println!("Invalid correctness mask");
				buf.clear();
			}
		};

		if guess.is_correct() {
			println!("You win!!");
			return;
		}

		w.guess(guess);
	}
}
