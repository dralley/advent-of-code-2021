use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").expect("failed to open file");
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|num| str::parse(&num).expect("failed to parse number"))
        .collect();

    println!("Solution 1: {}", count_increases(&numbers));

    let sliding_windows_sums: Vec<i32> = numbers
        .windows(3)
        .map(|w| w.iter().sum())
        .collect();

    println!("Solution 2: {}", count_increases(&sliding_windows_sums));
}

fn count_increases(nums: &[i32]) -> usize {
    nums
        .windows(2)
        .filter(|sl| sl[0] < sl[1])
        .count()
}
