use itertools::Itertools;

static ALPHABET_STR: &str = "abcdefghijklmnopqrstuvwxyz";

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if y == 0 {
                grid.push(vec![]);
            }
            grid[x].push(ch);
        }
    }
    grid
}

fn get_neighbours(x: &usize, y: &usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let x_min = if x > &0 { x - 1 } else { 0 };
    let y_min = if y > &0 { y - 1 } else { 0 };
    let x_max = if x < &(grid.len() - 1) as &usize {
        x + 1
    } else {
        *x
    };
    let y_max = if y < &(grid[0].len() - 1) { y + 1 } else { *y };

    let mut coords = vec![];

    for x1 in x_min..=x_max {
        for y1 in y_min..=y_max {
            if (*x, *y) != (x1, y1) {
                coords.push((x1, y1));
            }
        }
    }
    coords
}

fn print_visited(visited: &Vec<Vec<bool>>) {
    for i in 0..visited[0].len() {
        println!(
            "{}",
            visited
                .iter()
                .map(|col| if col[i] { "X" } else { "." })
                .collect::<String>()
        );
    }
    println!("-------");
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (x, col) in grid.iter().enumerate() {
        for (y, ch) in col.iter().enumerate() {
            if ch == &'S' {
                return (x, y);
            }
        }
    }
    unimplemented!()
}

fn is_valid_path(
    source: &QItem,
    x: isize,
    y: isize,
    grid: &Vec<Vec<char>>,
    visited: &Vec<Vec<bool>>,
) -> bool {
    if x < 0 || y < 0 {
        return false;
    };
    let x = x as usize;
    let y = y as usize;
    if x >= grid.len() || y >= grid[0].len() {
        return false;
    }

    let is_start = grid[source.x][source.y] == 'S';
    let is_next_end = grid[x][y] == 'E';
    let next_char_index = ALPHABET_STR.chars().position(|ch| grid[x][y] == ch);
    let curr_char_index = ALPHABET_STR
        .chars()
        .position(|ch| grid[source.x][source.y] == ch);
    let curr_char = curr_char_index.and_then(|i| ALPHABET_STR.chars().nth(i));
    // let next_char = next_char_index.and_then(|i| ALPHABET_STR.chars().nth(i));
    if (is_start
        || (is_next_end && curr_char.unwrap() == 'z')
        || (next_char_index.is_some()
            && next_char_index.unwrap() <= (curr_char_index.unwrap() + 1)))
        && !visited[x][y]
    {
        // println!("{x} {y} {:#?} {:#?}", curr_char, next_char);
        return true;
    }
    false
}

struct QItem {
    x: usize,
    y: usize,
    distance: usize,
}

fn find_shortest_path(grid: &Vec<Vec<char>>) -> usize {
    let mut queue = vec![];
    let mut visited = grid
        .clone()
        .iter()
        .map(|col| col.iter().map(|_| false).collect_vec())
        .collect_vec();

    let start = find_start(grid);
    queue.push(QItem {
        x: start.0,
        y: start.1,
        distance: 0,
    });
    visited[start.0][start.1] = true;

    while queue.len() > 0 {
        // print_visited(&visited);
        let source = queue.pop().unwrap();

        if grid[source.x][source.y] == 'E' {
            return source.distance;
        }

        // UP
        if is_valid_path(
            &source,
            source.x as isize,
            source.y as isize - 1,
            grid,
            &visited,
        ) {
            queue.push(QItem {
                x: source.x,
                y: source.y - 1,
                distance: source.distance + 1,
            });
            visited[source.x][source.y - 1] = true;
        }
        // DOWN
        if is_valid_path(
            &source,
            source.x as isize,
            source.y as isize + 1,
            grid,
            &visited,
        ) {
            queue.push(QItem {
                x: source.x,
                y: source.y + 1,
                distance: source.distance + 1,
            });
            visited[source.x][source.y + 1] = true;
        }
        // RIGHT
        if is_valid_path(
            &source,
            source.x as isize + 1,
            source.y as isize,
            grid,
            &visited,
        ) {
            queue.push(QItem {
                x: source.x + 1,
                y: source.y,
                distance: source.distance + 1,
            });
            visited[source.x + 1][source.y] = true;
        }
        // LEFT
        if is_valid_path(
            &source,
            source.x as isize - 1,
            source.y as isize,
            grid,
            &visited,
        ) {
            queue.push(QItem {
                x: source.x - 1,
                y: source.y,
                distance: source.distance + 1,
            });
            visited[source.x - 1][source.y] = true;
        }
    }

    dbg!(visited);
    todo!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let distance = find_shortest_path(&grid);
    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
