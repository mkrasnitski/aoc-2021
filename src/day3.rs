pub struct BitMatrix {
    nums: Vec<u32>,
    width: usize,
}

#[aoc_generator(day3)]
pub fn parse_report(input: &str) -> BitMatrix {
    let mut width = 0;
    let nums = input
        .lines()
        .map(|l| {
            let len = l.len();
            if len > width {
                width = len;
            }
            u32::from_str_radix(l, 2).unwrap()
        })
        .collect();
    BitMatrix { nums, width }
}

enum Majority {
    One,
    Zero,
    Tie,
}

fn majority(nums: &[u32], bit: usize) -> Majority {
    let mut total = 0;
    nums.iter().for_each(|n| match (n >> bit) & 1 != 0 {
        true => total += 1,
        false => total -= 1,
    });
    match total {
        std::i32::MIN..=-1 => Majority::Zero,
        0 => Majority::Tie,
        1.. => Majority::One,
    }
}

fn majority_filter(nums: &[u32], bit: usize, invert: bool) -> u32 {
    let mut filter_val = match majority(nums, bit) {
        Majority::Zero => 0,
        _ => 1,
    };

    if invert {
        filter_val = 1 - filter_val;
    }

    let filtered = nums
        .iter()
        .filter(move |&n| (n >> bit) & 1 == filter_val)
        .cloned()
        .collect::<Vec<u32>>();

    if filtered.len() == 1 {
        filtered[0]
    } else {
        majority_filter(&filtered, bit - 1, invert)
    }
}

#[aoc(day3, part1)]
pub fn part1(data: &BitMatrix) -> u32 {
    let mut gamma = 0;
    for i in 0..data.width {
        if let Majority::One = majority(&data.nums, i) {
            gamma |= 1 << i;
        }
    }
    let epsilon = !gamma & ((1 << data.width) - 1);
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(data: &BitMatrix) -> u32 {
    let o2 = majority_filter(&data.nums, data.width - 1, false);
    let co2 = majority_filter(&data.nums, data.width - 1, true);
    o2 * co2
}
