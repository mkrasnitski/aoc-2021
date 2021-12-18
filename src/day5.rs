use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Copy, Clone)]
pub struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

trait RangeIntersection: Sized {
    fn intersection(self, other: Self) -> Self;
}

impl<Idx> RangeIntersection for RangeInclusive<Idx>
where
    Idx: Ord + Copy,
{
    fn intersection(self, other: Self) -> Self {
        let max_start = self.start().max(other.start());
        let min_end = self.end().min(other.end());

        *max_start..=*min_end
    }
}

impl Line {
    fn intersect(self, other: &Line, set: &mut HashSet<Point>) {
        match (self.direction, other.direction) {
            (Direction::Horizontal, Direction::Horizontal) => {
                if self.start.y == other.start.y {
                    let range =
                        (self.start.x..=self.end.x).intersection(other.start.x..=other.end.x);
                    set.extend(range.map(|x| Point { x, y: self.start.y }))
                }
            }
            (Direction::Vertical, Direction::Vertical) => {
                if self.start.x == other.start.x {
                    let range =
                        (self.start.y..=self.end.y).intersection(other.start.y..=other.end.y);
                    set.extend(range.map(|y| Point { x: self.start.x, y }))
                }
            }
            (Direction::Horizontal, Direction::Vertical) => {
                set.insert(Point {
                    x: other.start.x,
                    y: self.start.y,
                });
            }
            (Direction::Vertical, Direction::Horizontal) => {
                set.insert(Point {
                    x: self.start.x,
                    y: other.start.y,
                });
            }
            _ => {}
        };
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIterator;
    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            current: None,
            start: self.start,
            end: self.end,
        }
    }
}

pub struct LineIterator {
    current: Option<Point>,
    start: Point,
    end: Point,
}

impl<'a> Iterator for LineIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.current = match self.current {
            None => Some(self.start),
            Some(prev) => {
                if prev == self.end {
                    None
                } else {
                    let x_diff = self.end.x as i32 - prev.x as i32;
                    let y_diff = self.end.y as i32 - prev.y as i32;
                    let x = (prev.x as i32 + x_diff.signum()) as u32;
                    let y = (prev.y as i32 + y_diff.signum()) as u32;
                    Some(Point { x, y })
                }
            }
        };
        self.current
    }
}

#[aoc_generator(day5)]
pub fn parse_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once(" -> ").unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            Line {
                start,
                end,
                direction: if start.x == end.x {
                    Direction::Vertical
                } else if start.y == end.y {
                    Direction::Horizontal
                } else {
                    Direction::Diagonal
                },
            }
        })
        .collect()
}

#[aoc(day5, part1, Iterate)]
pub fn part1_iterate(lines: &[Line]) -> u32 {
    let mut map = HashMap::new();
    let mut total = 0;
    for line in lines {
        if let Direction::Horizontal | Direction::Vertical = line.direction {
            for p in line.into_iter() {
                let count = map.entry(p).or_insert(0);
                *count += 1;
                if *count == 2 {
                    total += 1;
                }
            }
        }
    }
    total
}

#[aoc(day5, part1, Intersect)]
pub fn part1_intersect(lines: &[Line]) -> u32 {
    let mut set = HashSet::new();
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines.iter().skip(i + 1) {
            line1.intersect(line2, &mut set);
        }
    }
    set.len() as u32
}

#[aoc(day5, part2)]
pub fn part2(lines: &[Line]) -> u32 {
    let mut map = HashMap::new();
    let mut total = 0;
    for line in lines {
        for p in line.into_iter() {
            let count = map.entry(p).or_insert(0);
            *count += 1;
            if *count == 2 {
                total += 1;
            }
        }
    }
    total
}
