use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|inv| {
            inv.lines()
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .split("\n\n")
        .map(|inv| {
            inv.lines()
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .sum::<u32>();
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
