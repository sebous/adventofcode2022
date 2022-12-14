use std::{collections::HashMap, fmt};

use advent_of_code::helpers::{Coord, Grid};
use itertools::Itertools;

type Rocks = Vec<Vec<(usize, usize)>>;

enum Point {
    Air,
    Rock,
    Sand,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Point::Air => ".",
            Point::Rock => "#",
            Point::Sand => "+",
        };
        write!(f, "{}", str)
    }
}

fn parse(input: &str) -> Rocks {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord_str| {
                    let (x, y) = coord_str.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect_vec()
        })
        .collect_vec()
}

fn create_path(start: &(usize, usize), end: &(usize, usize)) -> Vec<(usize, usize)> {
    let axis = if start.0 != end.0 { Axis::X } else { Axis::Y };
    let reversed = match &axis {
        Axis::X => end.0 < start.0,
        Axis::Y => end.1 < start.1,
    };
    let head_path = match (&axis, reversed) {
        (Axis::X, false) => (start.0..=end.0).map(|x| (x, start.1)).collect_vec(),
        (Axis::X, true) => (end.0..=start.0).map(|x| (x, start.1)).rev().collect_vec(),
        (Axis::Y, false) => (start.1..=end.1).map(|y| (start.0, y)).collect_vec(),
        (Axis::Y, true) => (end.1..=start.1).map(|y| (start.0, y)).rev().collect_vec(),
    };
    head_path
}

#[derive(PartialEq)]
enum Axis {
    X,
    Y,
}

fn build_grid(data: &Rocks) -> Grid<Point> {
    let mut x_sorted = data
        .iter()
        .flat_map(|item| item.iter().map(|(x, _)| x))
        .sorted();
    let (x_min, x_max) = (x_sorted.next().unwrap(), x_sorted.last().unwrap());
    let mut y_sorted = data
        .iter()
        .flat_map(|item| item.iter().map(|(_, y)| y))
        .sorted();
    let (y_min, y_max) = (y_sorted.next().unwrap(), y_sorted.last().unwrap());

    let mut map = HashMap::new();

    for x in 0..=x_max * 2 {
        for y in 0..=*y_max + 2 {
            map.insert((x, y), Point::Air);
        }
    }

    for path in data {
        for path_segment in path.windows(2) {
            for (x, y) in create_path(&path_segment[0], &path_segment[1]) {
                map.insert((x, y), Point::Rock);
            }
        }
    }

    Grid {
        map,
        width: x_max - x_min,
        height: y_max - y_min,
    }
}

fn move_sand(grid: &Grid<Point>, (x, y): &Coord) -> Option<Coord> {
    let down = grid.map.get(&(*x, y + 1));
    match down {
        Some(pt) => match pt {
            Point::Air => return Some((*x, y + 1)),
            _ => {}
        },
        None => return None,
    }

    let left = grid.map.get(&(x - 1, y + 1));
    match left {
        Some(pt) => match pt {
            Point::Air => return Some((*x - 1, y + 1)),
            _ => {}
        },
        None => return None,
    }

    let right = grid.map.get(&(x + 1, y + 1));
    match right {
        Some(pt) => match pt {
            Point::Air => return Some((*x + 1, y + 1)),
            _ => return Some((*x, *y)),
        },
        None => return None,
    }
}

fn count_sand(grid: &mut Grid<Point>) -> u32 {
    let sand_start = (500, 0);
    let mut counter = 0;

    'outer: loop {
        let mut curent_coord = sand_start;
        loop {
            match move_sand(grid, &curent_coord) {
                Some(new_coord) => {
                    if new_coord == curent_coord {
                        grid.map.insert(new_coord, Point::Sand);
                        counter += 1;

                        if new_coord == sand_start {
                            break 'outer;
                        }

                        break;
                    }
                    curent_coord = new_coord;
                }
                None => break 'outer,
            }
        }
    }
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    let paths = parse(input);
    let mut grid = build_grid(&paths);
    let cnt = count_sand(&mut grid);
    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    let paths = parse(input);
    let mut grid = build_grid(&paths);

    let (_, y_max) = grid.get_max_coord();
    for (_, pt) in grid.map.iter_mut().filter(|((_, y), _)| *y == y_max) {
        *pt = Point::Rock
    }

    let cnt = count_sand(&mut grid);
    Some(cnt)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
