use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("{x_min}-{x_max},{y_min}-{y_max}")]
struct Assigment {
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let c = input
        .lines()
        .map(|line| line.parse::<Assigment>().unwrap())
        .filter(|ass| {
            if (ass.x_min >= ass.y_min && ass.x_max <= ass.y_max)
                || (ass.y_min >= ass.x_min && ass.y_max <= ass.x_max)
            {
                true
            } else {
                false
            }
        })
        .count();
    Some(c as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let c = input
        .lines()
        .map(|line| line.parse::<Assigment>().unwrap())
        .filter(|ass| {
            if (ass.x_max >= ass.y_min && ass.x_max <= ass.y_max)
                || (ass.y_min >= ass.x_min && ass.y_min <= ass.x_max)
                || (ass.x_min >= ass.y_min && ass.x_min <= ass.y_max)
                || (ass.y_max >= ass.x_min && ass.y_max <= ass.x_max)
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
