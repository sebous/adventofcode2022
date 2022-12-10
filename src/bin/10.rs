use std::collections::HashMap;

use itertools::Itertools;

fn process(input: &str) -> (HashMap<i32, i32>, Vec<bool>) {
    let mut history = HashMap::new();
    let mut pixels = vec![];
    let mut curr_value = 1;
    let mut cycle = 1;

    for line in input.lines() {
        match &line[..4] {
            "noop" => {
                let crt_index = (cycle - 1) % 40;
                pixels.push(crt_index >= curr_value - 1 && crt_index <= curr_value + 1);
                cycle += 1;
            }
            "addx" => {
                for _ in 0..2 {
                    let crt_index = (cycle - 1) % 40;
                    pixels.push(crt_index >= curr_value - 1 && crt_index <= curr_value + 1);
                    cycle += 1;
                }

                let (_, val) = line.split_once(" ").unwrap();
                curr_value += val.parse::<i32>().unwrap();
                history.insert(cycle, curr_value);
            }
            _ => unimplemented!(),
        }
    }
    (history, pixels)
}

const SIGNAL_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

pub fn part_one(input: &str) -> Option<i32> {
    let (history, _) = process(input);
    let signal_strengths: i32 = SIGNAL_CYCLES
        .map(|cycle| {
            history
                .iter()
                .filter(|(k, _)| *k <= &cycle)
                .sorted_by(|a, b| b.0.cmp(a.0))
                .map(|(_, v)| v * cycle)
                .next()
                .unwrap()
        })
        .iter()
        .sum();

    Some(signal_strengths)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, pixels) = process(input);
    for line in pixels.chunks(40) {
        println!(
            "{}",
            line.iter().map(|x| if *x { "#" } else { "." }).join("")
        )
    }
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(0));
    }
}
