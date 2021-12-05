use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::Ordering;


type Point = (u32, u32);

#[derive(Default, Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn covers_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        let (mut x, mut y) = self.start;

        loop {
            points.push((x, y));

            match x1.cmp(&x2) {
                Ordering::Less => x += 1,
                Ordering::Equal => {},
                Ordering::Greater => x -= 1,
            }

            match y1.cmp(&y2) {
                Ordering::Less => y += 1,
                Ordering::Equal => {},
                Ordering::Greater => y -= 1,
            }

            if (x, y) == (x2, y2) {
                points.push((x, y));
                break;
            }
        }

        points
    }
}

fn problem1(lines: &[LineSegment]) -> usize {
    let mut collection: HashMap<Point, u32> = HashMap::new();

    for segment in lines.iter().filter(|p| p.start.0 == p.end.0 || p.start.1 == p.end.1) {
        for point in segment.covers_points() {
            collection.entry(point).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    collection.values().filter(|&&v| v > 1).count() as usize
}

fn problem2(lines: &[LineSegment]) -> usize {
    let mut collection: HashMap<Point, u32> = HashMap::new();

    for segment in lines.iter() {
        for point in segment.covers_points() {
            collection.entry(point).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    collection.values().filter(|&&v| v > 1).count() as usize
}

fn parse_line_segment(input: &str) -> LineSegment {
    let (start, end) = input.split_once("->").unwrap();
    let (x1, y1) = start.split_once(',').unwrap();
    let (x2, y2) = end.split_once(',').unwrap();

    let x1: u32 = x1.trim().parse().unwrap();
    let y1: u32 = y1.trim().parse().unwrap();
    let x2: u32 = x2.trim().parse().unwrap();
    let y2: u32 = y2.trim().parse().unwrap();

    LineSegment {
        start: (x1, y1),
        end: (x2, y2),
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let lines: Vec<LineSegment> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|s| parse_line_segment(&s))
        .collect();

    println!("Solution 1: {}", problem1(&lines));
    println!("Solution 2: {}", problem2(&lines));
}
