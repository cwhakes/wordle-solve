use clap::{Parser, Subcommand};

use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

/// A program to solve wordles
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long)]
	guesser: Option<String>,
	#[clap(subcommand)]
	command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
	/// Solve wordles from list in `answers.txt`
	List {
		#[clap(short, long)]
		limit: Option<usize>,
	},
}

fn main() {
	let args = Args::parse();

	dbg!(&args.guesser);
	match &args.command.unwrap_or(Command::List { limit: None }) {
		Command::List { limit } => list(*limit),
	}
}

fn list(limit: Option<usize>) {
	let w = Wordle::new();
	let guesser = algorithms::Naive::new();
	for answer in ANSWERS.lines().take(limit.unwrap_or(usize::MAX)) {
		println!("{}:", answer);
		let count = w.play(answer, guesser.clone());
		if let Some(count) = count {
			println!("Guessed {} in {} tries", answer, count);
		} else {
			println!("Failed to guess {}", answer);
		}
	}
}
