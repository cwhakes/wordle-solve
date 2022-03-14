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
		limit: Option<usize>,
	},
	/// Give suggestions for daily wordle
	Solve,
}

#[derive(ArgEnum, Debug, Clone)]
enum GuesserArg {
	Naive,
}

fn main() {
	let args = Args::parse();

	dbg!(&args.guesser);
	match &args.command.unwrap_or(Command::List { limit: None }) {
		Command::List { limit } => match &args.guesser {
			GuesserArg::Naive => list(*limit, algorithm::Naive::default),
		},
		Command::Solve => match &args.guesser {
			GuesserArg::Naive => solve(algorithm::Naive::default),
		},
	}
}

fn list<G>(limit: Option<usize>, guesser: impl Fn() -> G)
where
	G: Guesser,
{
	let w = Wordle::default();
	let mut guesser = guesser();

	for answer in ANSWERS.lines().take(limit.unwrap_or(usize::MAX)) {
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

fn solve<G>(guesser: impl Fn() -> G)
where
	G: Guesser,
{
	let mut guesser = guesser();

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
