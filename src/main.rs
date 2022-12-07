use std::io::BufRead;

use clap::Parser;

mod day1;

#[derive(clap::Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    day: Day,
}

#[derive(clap::Subcommand)]
enum Day {
    Day1_1,
    Day1_2,
}

fn main() {
    let cli = Cli::parse();

    let stdin = std::io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cli.day {
        Day::Day1_1 => day1::solve1(&input),
        Day::Day1_2 => day1::solve2(&input),
    };

    println!("result: {result}")
}
