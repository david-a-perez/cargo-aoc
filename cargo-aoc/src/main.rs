mod app;
mod args;
mod credentials;
mod date;
mod project;
mod errors;

use aoc_runner_internal::{Day, Part};
use app::{execute_bench, execute_credentials, execute_default, execute_input};

use crate::args::args_without_aoc;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "cargo-aoc",
    version = "0.3.8",
    author = "gobanos <gregory.obanos@gmail.com>",
    about = "Cargo helper for Advent of Code"
)]
pub struct Cli {
    /// Specifies the day. Defaults to last implemented.
    #[clap(short, long)]
    day: Option<Day>,

    /// Specifies the part. Defaults to both parts.
    #[clap(short, long)]
    part: Option<Part>,
    /// Use an alternate input file.
    #[clap(short, long)]
    input: Option<String>,
    /// Add debug info for profiling tools.
    #[clap(long)]
    profile: bool,

    /// Generate the boilerplate for the given day.
    #[clap(long, short)]
    generate: bool,

    #[clap(subcommand)]
    subcmd: Option<SubCommands>,
}

#[derive(Parser, Debug)]
enum SubCommands {
    Bench(Bench),
    Credentials(Credentials),
    Input(Input),
}

/// Runs the benchmark for the last day (or a given day)
#[derive(Parser, Debug)]
pub struct Bench {
    /// Specifies the day. Defaults to last implemented.
    #[clap(short, long)]
    day: Option<Day>,

    /// Run the benchmark for all days
    #[clap(long, conflicts_with_all = ["day", "input"])]
    all_days: bool,

    /// Specifies the part. Defaults to both parts.
    #[clap(short, long)]
    part: Option<Part>,

    /// Use an alternate input file.
    #[clap(short, long)]
    input: Option<String>,

    /// Opens the benchmark information in the browser
    #[clap(short, long)]
    open: bool,

    /// Also benchmark generator functions.
    #[clap(short, long)]
    generator: bool,

    /// Add debug info for profiling tools.
    #[clap(long)]
    profile: bool,
}

/// Sets the session cookie
#[derive(Parser, Debug)]
pub struct Credentials {
    set: Option<String>,
}

/// Downloads the input for today (or a given day)
#[derive(Parser, Debug)]
pub struct Input {
    /// Specifies the day. Defaults to today's date.
    #[clap(short, long)]
    day: Option<Day>,

    /// Specifies the year. Defaults to the current year.
    #[clap(short, long)]
    year: Option<i32>,

    /// Downloads all possible inputs for a given year
    #[clap(short, long)]
    all: bool,

    /// Generate the boilerplate for the given day.
    #[clap(long, short)]
    generate: bool,
}

fn main() {
    let cli = Cli::parse_from(args_without_aoc());

    let Some(subcommand) = cli.subcmd else {
        return execute_default(&cli).unwrap();
    };

    match subcommand {
        SubCommands::Bench(arg) => execute_bench(&arg),
        SubCommands::Credentials(arg) => {
            execute_credentials(&arg);
            Ok(())
        }
        SubCommands::Input(arg) => execute_input(&arg),
    }
    .unwrap()
}
