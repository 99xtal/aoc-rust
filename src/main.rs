use clap::{Parser, Subcommand};
use y2024 as Year2024;

mod y2024;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    year: Year
}

#[derive(Subcommand)]
enum Year {
    #[command(subcommand, name = "2024")]
    Y2024(Day2024),
}

#[derive(Subcommand)]
enum Day2024 {
    /// Find the total distance between the left list and right list of a location ID input
    #[command(name = "day1")]
    Day1(Year2024::day01::Day1Args),
}

fn main() {
    let cli = Cli::parse();

    match cli.year {
        Year::Y2024(day) => match day {
            Day2024::Day1(args) => {
                y2024::day01::run(args);
            }
        }
    }
}
