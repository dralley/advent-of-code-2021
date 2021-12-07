use std::fs::File;
use std::io::{BufRead, BufReader};


fn problem1(positions: &[usize]) -> usize {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut best: usize = usize::MAX;

    for i in min..=max {
        let mut result: usize = 0;

        for pos in positions {
            let diff = (*pos as isize - i as isize).abs() as usize;
            result += diff;
        }

        if result < best {
            best = result;
        }
    }

    best
}

fn problem2(positions: &[usize]) -> usize {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut best: usize = usize::MAX;

    for i in min..=max {
        let mut result: usize = 0;

        for pos in positions {
            let diff = (*pos as isize - i as isize).abs() as usize;
            result += (0..=diff).sum::<usize>();
        }

        if result < best {
            best = result;
        }
    }

    best
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let line: String = reader.lines().filter_map(|line| line.ok()).next().unwrap();

    let positions: Vec<usize> = line
        .split(',')
        .map(|c| str::parse(c).unwrap())
        .collect();

    println!("Solution 1: {}", problem1(&positions));
    println!("Solution 2: {}", problem2(&positions));
}
