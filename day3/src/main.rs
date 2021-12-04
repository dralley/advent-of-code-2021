use std::fs::File;
use std::io::{BufRead, BufReader};

fn problem1(column_wise_bits: &[Vec<char>]) -> u32 {
    let gamma_bits: String = column_wise_bits
        .iter()
        .map(|col| most_common_bit(&col))
        .collect();
    let gamma = u32::from_str_radix(&gamma_bits, 2).unwrap();

    let epsilon_bits: String = column_wise_bits
        .iter()
        .map(|col| least_common_bit(&col))
        .collect();
    let epsilon = u32::from_str_radix(&epsilon_bits, 2).unwrap();

    gamma * epsilon
}

fn problem2(column_wise: &Vec<Vec<char>>) -> u32 {
    let oxygen_rating = filter_report(column_wise, true);
    let co2_rating = filter_report(column_wise, false);
    oxygen_rating * co2_rating
}

fn filter_report(column_wise: &Vec<Vec<char>>, which_bit: bool) -> u32 {
    let width = column_wise.len();
    let mut column_wise = column_wise.clone();

    for column in 0..width {
        let significant_bit = if which_bit {
            most_common_bit(&column_wise[column])
        } else {
            least_common_bit(&column_wise[column])
        };

        let rows_to_keep: Vec<usize> = column_wise[column]
            .iter()
            .enumerate()
            .filter(|(row, &bit)| bit == significant_bit)
            .map(|(row, _)| row)
            .collect();
        for _column in 0..width {
            column_wise[_column] = column_wise[_column]
                .iter()
                .enumerate()
                .filter(|(row, &bit)| rows_to_keep.contains(row))
                .map(|(_, &bit)| bit)
                .collect();
        }

        if column_wise[0].len() == 1 {
            break;
        }
    }

    let bits: String = column_wise.iter().map(|col| col[0]).collect();

    u32::from_str_radix(&bits, 2).unwrap()
}

fn most_common_bit(bits: &[char]) -> char {
    if bits.iter().filter(|&&c| c == '1').count() >= (bits.len() / 2) {
        '1'
    } else {
        '0'
    }
}

fn least_common_bit(bits: &[char]) -> char {
    if bits.iter().filter(|&&c| c == '1').count() >= (bits.len() / 2) {
        '0'
    } else {
        '1'
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    let mut column_wise_bits = Vec::new();

    for column in 0..lines[0].len() {
        let new_column = lines
            .iter()
            .map(|line| line.chars().nth(column).unwrap())
            .collect();
        column_wise_bits.push(new_column);
    }

    println!("Solution 1: {}", problem1(&column_wise_bits));
    println!("Solution 2: {}", problem2(&column_wise_bits));
}
