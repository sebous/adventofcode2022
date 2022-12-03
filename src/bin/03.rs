use itertools::Itertools;
static ALPHABET_STR: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn part_one(input: &str) -> Option<u32> {
    let alphabet = ALPHABET_STR.chars().collect_vec();
    let total = input
        .lines()
        .map(|line| {
            let line = line.chars().collect_vec();
            let half_index = line.len() / 2;
            let left = &line[..half_index];
            let right = &line[half_index..];
            let matching_char = left.iter().find(|x| right.contains(x)).unwrap();
            let is_upper = matching_char.is_uppercase();
            let (alph_i, _) = alphabet
                .iter()
                .find_position(|x| x.to_string() == matching_char.to_lowercase().to_string())
                .unwrap();
            let value = alph_i as u32 + 1 + if is_upper { 26 } else { 0 };
            value
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let alphabet = ALPHABET_STR.chars().collect_vec();
    let total = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .chunks(3)
        .into_iter()
        .map(|mut x| {
            let first = x.next().unwrap();
            let second = x.next().unwrap();
            let third = x.next().unwrap();
            let matching_char = first
                .iter()
                .find(|ch| second.contains(ch) && third.contains(ch))
                .unwrap();
            let (alph_i, _) = alphabet
                .iter()
                .find_position(|x| x.to_string() == matching_char.to_lowercase().to_string())
                .unwrap();
            let value = alph_i as u32 + 1 + if matching_char.is_uppercase() { 26 } else { 0 };
            value
        })
        .sum::<u32>();
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
