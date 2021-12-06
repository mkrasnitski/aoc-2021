use std::collections::HashMap;

#[derive(Clone)]
pub struct Bingo {
    draw_order: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    board: HashMap<u32, (usize, usize)>,
    marks: [[bool; 5]; 5],
}

impl Board {
    fn mark(&mut self, num: &u32) -> bool {
        if let Some(&(i, j)) = self.board.get(&num) {
            self.marks[i][j] = true;
            if self.marks[i].into_iter().all(|m| m)
                || self.marks.into_iter().map(|row| row[j]).all(|m| m)
            {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for (&n, &(i, j)) in self.board.iter() {
            if !self.marks[i][j] {
                score += n;
            }
        }
        score
    }
}

#[aoc_generator(day4)]
pub fn parse_bingo(input: &str) -> Bingo {
    let mut lines = input.lines();
    let draw_order = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut boards = vec![];
    while let Some("") = lines.next() {
        let mut hashmap = HashMap::new();
        for i in 0..5 {
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(j, n)| {
                    hashmap.insert(n, (i, j));
                });
        }
        boards.push(Board {
            board: hashmap,
            marks: [[false; 5]; 5],
        });
    }
    Bingo { draw_order, boards }
}

#[aoc(day4, part1)]
pub fn part1(b: &Bingo) -> u32 {
    let mut b = (*b).clone();
    for num in b.draw_order.iter() {
        for board in b.boards.iter_mut() {
            if board.mark(num) {
                return num * board.score();
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(b: &Bingo) -> u32 {
    let mut b = (*b).clone();
    let mut score = 0;
    for num in b.draw_order.iter() {
        let mut removal_indices = vec![];
        for (i, board) in b.boards.iter_mut().enumerate() {
            if board.mark(num) {
                score = num * board.score();
                removal_indices.push(i);
            }
        }
        removal_indices.sort();
        for i in removal_indices.into_iter().rev() {
            b.boards.remove(i);
        }
    }
    score
}
