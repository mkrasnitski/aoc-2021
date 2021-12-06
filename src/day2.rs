enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Command {
    direction: Direction,
    distance: u32,
}

#[aoc_generator(day2)]
pub fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let (direction, distance) = l.split_once(' ').unwrap();
            Command {
                direction: match direction {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => unreachable!(),
                },
                distance: distance.parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(commands: &[Command]) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    for command in commands {
        let d = command.distance;
        match command.direction {
            Direction::Forward => position += d,
            Direction::Down => depth += d,
            Direction::Up => depth -= d,
        }
    }
    position * depth
}

#[aoc(day2, part2)]
pub fn part2(commands: &[Command]) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        let d = command.distance;
        match command.direction {
            Direction::Forward => {
                position += d;
                depth += aim * d;
            }
            Direction::Down => aim += d,
            Direction::Up => aim -= d,
        }
    }
    position * depth
}
