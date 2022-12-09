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

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited_coords = HashSet::from([(0, 0)]);
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    for line in input.lines() {
        // println!("--- {} ---", &line);
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
        // println!("head path: {:?}", &head_path);

        for pos in head_path {
            head_pos = pos;
            // println!("H {:?}", head_pos);
            if !should_move(&head_pos, &tail_pos) {
                // println!("T {:?}", tail_pos);
                continue;
            }
            let (x, y) = tail_pos;

            let x_move = if head_pos.0 > x { 1 } else { -1 };
            let y_move = if head_pos.1 > y { 1 } else { -1 };
            if head_pos.1 == y {
                tail_pos = (x + x_move, y);
            } else if head_pos.0 == x {
                tail_pos = (x, y + y_move);
            } else {
                tail_pos = (x + x_move, y + y_move);
            }
            visited_coords.insert(tail_pos);

            // println!("T {:?}", tail_pos);
        }
    }

    Some(visited_coords.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
