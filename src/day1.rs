#[aoc_generator(day1)]
pub fn parse_nums(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn increasing_window_sum(nums: &[u32], size: usize) -> u32 {
    nums.windows(size + 1).filter(|w| w[0] < w[size]).count() as u32
}

#[aoc(day1, part1)]
pub fn part1(data: &[u32]) -> u32 {
    increasing_window_sum(data, 1)
}

#[aoc(day1, part2)]
pub fn part2(data: &[u32]) -> u32 {
    increasing_window_sum(data, 3)
}
