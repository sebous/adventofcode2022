use std::collections::HashSet;

use itertools::Itertools;

fn should_move(a: &(i32, i32), b: &(i32, i32)) -> bool {
    (a.0 - b.0).abs() > 1 || (a.1 - b.1).abs() > 1
}

fn create_path(start: &(i32, i32), end: &(i32, i32)) -> Vec<(i32, i32)> {
    let axis = if start.0 != end.0 { Axis::X } else { Axis::Y };
    let reversed = match &axis {
        Axis::X => end.0 < start.0,
        Axis::Y => end.1 < start.1,
    };
    let head_path = match (&axis, reversed) {
        (Axis::X, false) => (start.0 + 1..end.0 + 1).map(|x| (x, start.1)).collect_vec(),
        (Axis::X, true) => (end.0..start.0).map(|x| (x, start.1)).rev().collect_vec(),
        (Axis::Y, false) => (start.1 + 1..end.1 + 1).map(|y| (start.0, y)).collect_vec(),
        (Axis::Y, true) => (end.1..start.1).map(|y| (start.0, y)).rev().collect_vec(),
    };
    head_path
}

#[derive(PartialEq)]
enum Axis {
    X,
    Y,
}

fn print_tails(tails: &Vec<(i32, i32)>) {
    for y in (-10..20).rev() {
        let line: String = (-20..30)
            .map(|x| {
                tails
                    .iter()
                    .position(|tail| *tail == (x, y))
                    .map(|num| (num + 1).to_string())
                    .or_else(|| {
                        if (x, y) == (0, 0) {
                            Some("s".to_string())
                        } else {
                            Some(".".to_string())
                        }
                    })
            })
            .filter_map(|s| s)
            .collect();
        println!("{line}");
    }
}

fn solve(input: &str, tail_size: u8) -> HashSet<(i32, i32)> {
    let mut visited_coords = HashSet::new();
    let mut head_pos = (0, 0);
    let mut tails = (0..tail_size).map(|_| (0, 0)).collect_vec();

    for line in input.lines() {
        let (dir, val) = line.split_once(" ").unwrap();
        let val: i32 = val.parse().unwrap();

        let next_head_pos = match dir {
            "R" => (head_pos.0 + val, head_pos.1),
            "L" => (head_pos.0 - val, head_pos.1),
            "U" => (head_pos.0, head_pos.1 + val),
            "D" => (head_pos.0, head_pos.1 - val),
            _ => unimplemented!(),
        };

        let head_path = create_path(&head_pos, &next_head_pos);

        for pos in head_path {
            for i in 0..tails.len() {
                let mut prev_pos = pos;
                if i == 0 {
                    head_pos = pos;
                } else {
                    prev_pos = tails[i - 1];
                }

                let tail_pos = tails[i];

                if !should_move(&prev_pos, &tail_pos) {
                    continue;
                }
                let (x, y) = tail_pos;

                let x_move = if prev_pos.0 > x { 1 } else { -1 };
                let y_move = if prev_pos.1 > y { 1 } else { -1 };
                if prev_pos.1 == y {
                    tails[i] = (x + x_move, y);
                } else if prev_pos.0 == x {
                    tails[i] = (x, y + y_move);
                } else {
                    tails[i] = (x + x_move, y + y_move);
                }
                if i == tails.len() - 1 {
                    visited_coords.insert(tail_pos);
                }
            }
        }
        // println!("--- {} ---", &line);
        // print_tails(&tails);
    }
    visited_coords
}

pub fn part_one(input: &str) -> Option<u32> {
    let visited_coords = solve(input, 1);
    Some((visited_coords.len() + 1) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let visited_coords = solve(input, 9);
    Some((visited_coords.len() + 1) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
