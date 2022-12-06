use itertools::Itertools;

fn find_marker(size: usize, input: &str) -> Option<usize> {
    for i in 0..input.len() {
        if input[i..i + size].chars().all_unique() {
            return Some(i + size);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    find_marker(4, input)
}

pub fn part_two(input: &str) -> Option<usize> {
    find_marker(14, input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
