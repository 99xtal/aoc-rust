use clap::Args;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Args, Debug)]
pub struct Day1Args {
    /// Filepath of .txt file containing location ID lists
    pub input: std::path::PathBuf,

    /// Use similarity score algorithm
    #[arg(short = 's', long = "score")]
    pub score: bool,
}

pub fn run(args: Day1Args) {
    let f = File::open(args.input).expect("Failed to open file.");

    let (list1, list2) = parse_lists(f);

    let value: i32;
    if args.score {
        value = get_similarity_score(&list1, &list2);
    } else {
        value = get_total_distance(&list1, &list2);
    }

    println!("{value}");
}

fn parse_lists(file: impl Read) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let nums = line.split_whitespace();
                let nums: Vec<i32> = nums.map(|s| s.parse().unwrap()).collect();
                list1.push(nums[0]);
                list2.push(nums[1]);
            }
            Err(_) => {}
        }
    }

    (list1, list2)
}

fn get_total_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut list1 = list1.to_vec();
    let mut list2 = list2.to_vec();

    list1.sort();
    list2.sort();

    let mut total_distance: i32 = 0;
    for i in 0..list1.len() {
        let distance = list1[i] - list2[i];
        total_distance += distance.abs();
    }

    total_distance
}

fn get_similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    for val in list2 {
        let count = freq_map.entry(*val).or_insert(0);
        *count += 1;
    }

    let mut score = 0;

    for val in list1 {
        if freq_map.contains_key(val) {
            score += *val * freq_map.get(val).copied().unwrap();
        }
    }

    score
}
