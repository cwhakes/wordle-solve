use std::io::{stdin, stdout, Write};

use clap::{Parser, Subcommand};

use wordle_solve::*;

const ANSWERS: &str = include_str!("../answers.txt");

/// A program to solve wordles
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long, default_value = "naive")]
	guesser: String,
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
	/// Give suggestions for daily wordle
	Solve,
}

fn main() {
	let args = Args::parse();

	dbg!(&args.guesser);
	match &args.command.unwrap_or(Command::List { limit: None }) {
		Command::List { limit } => list(*limit, &args.guesser),
		Command::Solve => solve(&args.guesser),
	}
}

fn list(limit: Option<usize>, guesser: &str) {
	let w = Wordle::default();

	let mut guesser = if let Some(guesser) = algorithm::select(guesser) {
		guesser
	} else {
		eprintln!("Unknown guesser `{}`", guesser);
		return;
	};

	for answer in ANSWERS.lines().take(limit.unwrap_or(usize::MAX)) {
		println!("{}:", answer);
		let count = w.play(answer, &mut *guesser);
		if let Some(count) = count {
			println!("Guessed {} in {} tries", answer, count);
		} else {
			println!("Failed to guess {}", answer);
			panic!();
		}
		guesser.reset();
	}
}

fn solve(guesser: &str) {
	let mut guesser = if let Some(guesser) = algorithm::select(guesser) {
		guesser
	} else {
		eprintln!("Unknown guesser `{}`", guesser);
		return;
	};

	let mut history = Vec::new();
	for _ in 1..=6 {
		let guess = guesser.guess(&history);
		println!("Guess: {}", guess);

		print!("Correctness: ");
		stdout().flush().unwrap();
		let mut buf = String::new();
		stdin().read_line(&mut buf).unwrap();
		let correctness = Correctness::new(&buf);

		if correctness == Correctness::new("CCCCC") {
			println!("You win!!");
			return;
		}

		let guess = Guess {
			word: guess,
			mask: correctness,
		};
		history.push(guess);
	}
}
