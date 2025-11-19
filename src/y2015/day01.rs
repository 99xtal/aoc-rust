use std::fs;

use clap::Args;

#[derive(Args, Debug)]
pub struct Day1Args {
    /// Filepath of .txt file containing building instructions
    pub input: std::path::PathBuf,

    /// Find index of first basement move
    #[arg(short = 'b')]
    pub basement: bool,
}

pub fn run(args: Day1Args) {
    let input = fs::read_to_string(args.input).expect("Failed to read input file");

    let value: i32;
    if args.basement {
        value = get_first_basement_pos(&input);
    } else {
        value = get_final_floor(&input);
    }

    println!("{value}");
}

fn get_final_floor(instructions: &str) -> i32 {
    let mut floor = 0;

    for c in instructions.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    floor
}

fn get_first_basement_pos(instructions: &str) -> i32 {
    let mut floor = 0;

    for (i, c) in instructions.char_indices() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            return i as i32 + 1;
        }
    }

    return 0;
}
