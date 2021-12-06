use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve(starting_ages: &[usize], iterations: u32) -> usize {
    let mut fish = [0usize; 9];

    for &age in starting_ages {
        fish[age] += 1;
    }

    for _ in 0..iterations {
        let fish_that_reproduced = fish[0];
        fish.rotate_left(1);
        fish[6] += fish_that_reproduced;
    }

    fish.iter().sum()
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let ages: Vec<usize> = reader.lines()
        .filter_map(|line| line.ok())
        .nth(0)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    println!("Solution 1: {}", solve(&ages, 80));
    println!("Solution 2: {}", solve(&ages, 256));
}
