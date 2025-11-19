use clap::{Parser, Subcommand};
use y2015 as Year2015;
use y2024 as Year2024;

mod y2015;
mod y2024;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    year: Year,
}

#[derive(Subcommand)]
enum Year {
    #[command(subcommand, name = "2015")]
    Y2015(Day2015),

    #[command(subcommand, name = "2024")]
    Y2024(Day2024),
}

#[derive(Subcommand)]
enum Day2015 {
    #[command(name = "day1")]
    Day1(Year2015::day01::Day1Args),

    #[command(name = "day2")]
    Day2(Year2015::day02::Day2Args),
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
        Year::Y2015(day) => match day {
            Day2015::Day1(args) => y2015::day01::run(args),
            Day2015::Day2(args) => y2015::day02::run(args),
        },
        Year::Y2024(day) => match day {
            Day2024::Day1(args) => y2024::day01::run(args),
        },
    }
}
