use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use once_cell::sync::OnceCell;

#[derive(Clone)]
struct BingoBoard {
    inner: [[u32; 5]; 5],
}

impl BingoBoard {
    fn new(inner: [[u32; 5]; 5]) -> Self {
        Self {
            inner: inner,
        }
    }

    fn winning_combinations() -> &'static Vec<Vec<(usize, usize)>> {
        static WINNING_COMBINATIONS: OnceCell<Vec<Vec<(usize, usize)>>> = OnceCell::new();
        WINNING_COMBINATIONS.get_or_init(|| {
            let mut winning_combinations = Vec::new();

            // rows
            for row in 0..5 {
                let row = iter::repeat(row).take(5).zip(0..5).collect();
                winning_combinations.push(row);
            }
            // columns
            for column in 0..5 {
                let column = (0..5).zip(iter::repeat(column).take(5)).collect();
                winning_combinations.push(column);
            }

            winning_combinations
        })
    }

    fn has_won(&self, drawn_numbers: &[u32]) -> Option<u32> {
        if Self::winning_combinations().iter().any(|combo| self.test_combination(combo, drawn_numbers)) {
            Some(self.score(drawn_numbers))
        } else {
            None
        }
    }

    fn test_combination(&self, combination: &[(usize, usize)], drawn_numbers: &[u32]) -> bool {
        combination
            .iter()
            .all(|&(row, column)| drawn_numbers.contains(&self.inner[row][column]))
    }

    fn score(&self, drawn_numbers: &[u32]) -> u32 {
        let sum_of_unmarked_numbers: u32 = self
            .inner
            .iter()
            .flatten()
            .filter(|num| !drawn_numbers.contains(num))
            .sum();
        let last_called_number = drawn_numbers.iter().last().unwrap();

        sum_of_unmarked_numbers * last_called_number
    }
}

fn problem1(numbers: &[u32], boards: &[BingoBoard]) -> u32 {
    let mut drawn_numbers: Vec<u32> = Vec::new();
    for number in numbers.iter().cloned() {
        drawn_numbers.push(number);
        let winning_score = boards
            .iter()
            .filter_map(|b| b.has_won(&drawn_numbers))
            .max();
        match winning_score {
            Some(score) => return score,
            None => continue,
        }
    }

    unreachable!("A winner should have been found before all numbers were called.")
}

fn problem2(numbers: &[u32], boards: &[BingoBoard]) -> u32 {
    let mut drawn_numbers: Vec<u32> = Vec::new();
    let mut scores: Vec<u32> = Vec::new();
    let mut boards = boards.to_vec();

    for number in numbers.iter().cloned() {
        drawn_numbers.push(number);

        boards.retain(|b| {
            if let Some(score) = b.has_won(&drawn_numbers) {
                scores.push(score);
                false
            } else {
                true
            }
        })
    }

    *scores.iter().last().unwrap()
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).filter(|l| !l.is_empty()).collect();

    let drawn_numbers: Vec<u32> = lines.remove(0)
        .split(',')
        .map(|c| str::parse(c).unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for chunk in lines.chunks_exact(5) {
        let mut board = [[0u32; 5]; 5];
        for (row, line) in chunk.iter().enumerate() {
            for (column, num) in line.split_whitespace().enumerate() {
                board[column][row] = str::parse(num).unwrap();
            }
        }

        boards.push(BingoBoard::new(board));
    }

    println!("Solution 1: {}", problem1(&drawn_numbers, &boards));
    println!("Solution 2: {}", problem2(&drawn_numbers, &boards));
}
