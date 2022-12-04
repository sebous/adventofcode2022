fn parse_input(input: &str) -> impl Iterator<Item = (u32, u32, u32, u32)> + '_ {
    input.lines().map(|line| {
        let (x, y) = line.split_once(",").unwrap();
        let (x_min, x_max) = x.split_once("-").unwrap();
        let (y_min, y_max) = y.split_once("-").unwrap();
        (
            x_min.parse().unwrap(),
            x_max.parse().unwrap(),
            y_min.parse().unwrap(),
            y_max.parse().unwrap(),
        )
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let c = parse_input(input)
        .filter(|ass| {
            let (x_min, x_max, y_min, y_max) = ass;
            if (x_min >= y_min && x_max <= y_max) || (y_min >= x_min && y_max <= x_max) {
                true
            } else {
                false
            }
        })
        .count();
    Some(c as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let c = parse_input(input)
        .filter(|ass| {
            let (x_min, x_max, y_min, y_max) = ass;
            if (x_max >= y_min && x_max <= y_max)
                || (y_min >= x_min && y_min <= x_max)
                || (x_min >= y_min && x_min <= y_max)
                || (y_max >= x_min && y_max <= x_max)
            {
                true
            } else {
                false
            }
        })
        .count();
    Some(c as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
