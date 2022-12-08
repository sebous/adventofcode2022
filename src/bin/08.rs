use std::{char, collections::HashMap};

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

fn create_grid(input: &str) -> (HashMap<(usize, usize), u8>, usize, usize) {
    let mut grid: HashMap<(usize, usize), u8> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x, y), char::to_digit(ch, 10).unwrap() as u8);
        }
    }

    let (width, height) = grid
        .keys()
        .max_by_key(|k| k.to_owned())
        .map(|(x, y)| (x + 1, y + 1))
        .unwrap();

    (grid, width, height)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, width, height) = create_grid(input);

    let mut visible_cnt = 0;

    for ((x1, y1), h1) in grid.iter() {
        let left = (0..*x1).all(|x2| grid.get(&(x2, *y1)).unwrap() < h1);
        let right = if x1 + 1 == width {
            true
        } else {
            (x1 + 1..width).all(|x2| grid.get(&(x2, *y1)).unwrap() < h1)
        };
        let top = (0..*y1).all(|y2| grid.get(&(*x1, y2)).unwrap() < h1);
        let bot = if y1 + 1 == height {
            true
        } else {
            (y1 + 1..height).all(|y2| grid.get(&(*x1, y2)).unwrap() < h1)
        };
        if left || right || top || bot {
            visible_cnt += 1;
        }
    }

    Some(visible_cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, width, height) = create_grid(input);
    let mut max_dist = 0;

    for ((x1, y1), h1) in grid.iter() {
        let left = (0..*x1)
            .rev()
            .fold_while(0, |total, x2| {
                if grid.get(&(x2, *y1)).unwrap() < h1 {
                    Continue(total + 1)
                } else {
                    Done(total + 1)
                }
            })
            .into_inner();
        let right = if x1 + 1 == width {
            0
        } else {
            (*x1 + 1..width)
                .fold_while(0, |total, x2| {
                    if grid.get(&(x2, *y1)).unwrap() < h1 {
                        Continue(total + 1)
                    } else {
                        Done(total + 1)
                    }
                })
                .into_inner()
        };
        let top = (0..*y1)
            .rev()
            .fold_while(0, |total, y2| {
                if grid.get(&(*x1, y2)).unwrap() < h1 {
                    Continue(total + 1)
                } else {
                    Done(total + 1)
                }
            })
            .into_inner();
        let bot = if y1 + 1 == height {
            0
        } else {
            (*y1 + 1..height)
                .fold_while(0, |total, y2| {
                    if grid.get(&(*x1, y2)).unwrap() < h1 {
                        Continue(total + 1)
                    } else {
                        Done(total + 1)
                    }
                })
                .into_inner()
        };

        let dist = left * right * top * bot;
        if max_dist > dist {
            continue;
        } else {
            max_dist = dist
        };
    }
    Some(max_dist as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
