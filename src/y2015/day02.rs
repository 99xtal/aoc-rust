use std::fs;

use clap::Args;

#[derive(Args, Debug)]
pub struct Day2Args {
    /// Filepath of .txt file containing box dimensions
    pub input: std::path::PathBuf,

    /// Get total ribbon length
    #[arg(short = 'r')]
    pub ribbon: bool,
}

pub fn run(args: Day2Args) {
    let input = fs::read_to_string(args.input).expect("Failed to read input file");

    let boxes = parse_input(&input);

    let eval: fn(&RectPrism) -> u32;
    if args.ribbon {
        eval = get_ribbon_length;
    } else {
        eval = get_wrapping_area;
    }

    let mut value = 0;
    for giftbox in boxes {
        value += eval(&giftbox);
    }

    println!("{value}");
}

fn parse_input(input: &str) -> Vec<RectPrism> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('x');

            let l: u32 = parts.next()?.parse().ok()?;
            let w: u32 = parts.next()?.parse().ok()?;
            let h: u32 = parts.next()?.parse().ok()?;

            Some(RectPrism { l, w, h })
        })
        .collect()
}

fn get_wrapping_area(gift: &RectPrism) -> u32 {
    let smallest_side = gift.smallest_side();

    return gift.surface_area() + smallest_side.0 * smallest_side.1;
}

fn get_ribbon_length(gift: &RectPrism) -> u32 {
    gift.smallest_side().perimeter() + gift.volume()
}

struct Rect(u32, u32);

impl Rect {
    fn perimeter(&self) -> u32 {
        self.0 + self.0 + self.1 + self.1
    }
}

struct RectPrism {
    l: u32,
    w: u32,
    h: u32,
}

impl RectPrism {
    fn surface_area(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.l * self.h
    }

    fn volume(&self) -> u32 {
        &self.h * &self.l * &self.w
    }

    fn smallest_side(&self) -> Rect {
        let mut sides = Vec::from([self.w, self.h, self.l]);
        sides.sort();
        Rect(sides[0], sides[1])
    }
}
