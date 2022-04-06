use clap::{ArgEnum, Parser, Subcommand};
use std::io::{stdin, stdout, Write};

use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

/// A program to solve wordles
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long, arg_enum, default_value_t = GuesserArg::Naive)]
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
	Naive,
}

fn main() {
	let args = Args::parse();

	dbg!(&args.guesser);
	match &args.command.unwrap_or_default() {
		Command::List { max } => match &args.guesser {
			GuesserArg::Naive => list(*max, algorithm::Naive::default),
		},
		Command::Cheat => match &args.guesser {
			&GuesserArg::Naive => cheat(algorithm::Naive::default),
		},
	}
}

fn list<G>(max: Option<usize>, guesser: impl Fn() -> G)
where
	G: Guesser,
{
	let w = Wordle::default();
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
		guesser.reset();
	}
}

fn cheat<G>(guesser: impl Fn() -> G)
where
	G: Guesser,
{
	let w = Wordle::default();
	let mut guesser = guesser();
	let mut history = Vec::new();
	for _ in 1..=6 {
		println!();
		let reccomendation = guesser.guess(&history);
		println!("Reccomendation: {}", reccomendation);

		let mut buf = String::new();
		let guess = loop {
			print!("      Guess: ");
			stdout().flush().unwrap();
			stdin().read_line(&mut buf).unwrap();

			if w.dictionary.contains(buf.trim()) {
				break buf.trim();
			} else {
				println!("Guess not in dictionary");
				buf.clear();
			}
		};

		let mut buf = String::new();
		let correctness = loop {
			print!("Correctness: ");
			stdout().flush().unwrap();
			stdin().read_line(&mut buf).unwrap();
			if let Some(c) = Correctness::new(buf.trim()) {
				break c;
			} else {
				println!("Invalid correctness mask");
				buf.clear();
			}
		};

		if correctness == Correctness::new("CCCCC").unwrap() {
			println!("You win!!");
			return;
		}

		let guess = Guess::new(guess, correctness);
		history.push(guess);
	}
}
