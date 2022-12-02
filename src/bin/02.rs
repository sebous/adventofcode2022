use itertools::Itertools;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn play_round((shape1, shape2): (&Shape, &Shape)) -> (u32, u32) {
    match shape1 {
        Shape::Rock => match shape2 {
            Shape::Rock => (4, 4),
            Shape::Paper => (1, 8),
            Shape::Scissors => (7, 3),
        },
        Shape::Paper => match shape2 {
            Shape::Rock => (8, 1),
            Shape::Paper => (5, 5),
            Shape::Scissors => (2, 9),
        },
        Shape::Scissors => match shape2 {
            Shape::Rock => (3, 7),
            Shape::Paper => (9, 2),
            Shape::Scissors => (6, 6),
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rounds = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|ch| match ch {
                    "A" | "X" => Shape::Rock,
                    "B" | "Y" => Shape::Paper,
                    "C" | "Z" => Shape::Scissors,
                    _ => unimplemented!(),
                })
                .take(2)
                .collect_tuple::<(Shape, Shape)>()
                .unwrap()
        })
        .collect_vec();

    let res = rounds.iter().fold(0, |total, (p1, p2)| {
        let (_, score_2) = play_round((p1, p2));
        total + score_2
    });
    Some(res)
}

enum RoundResult {
    Win,
    Loss,
    Draw,
}

fn select_round_result(shape: &Shape, wanted_result: RoundResult) -> Shape {
    match shape {
        Shape::Rock => match wanted_result {
            RoundResult::Win => Shape::Paper,
            RoundResult::Loss => Shape::Scissors,
            _ => Shape::Rock,
        },
        Shape::Paper => match wanted_result {
            RoundResult::Win => Shape::Scissors,
            RoundResult::Loss => Shape::Rock,
            _ => Shape::Paper,
        },
        Shape::Scissors => match wanted_result {
            RoundResult::Win => Shape::Rock,
            RoundResult::Loss => Shape::Paper,
            _ => Shape::Scissors,
        },
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let score: u32 = input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" ").unwrap();
            let p1_shape = match p1 {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unimplemented!(),
            };
            let p2_round_res = match p2 {
                "X" => RoundResult::Loss,
                "Y" => RoundResult::Draw,
                "Z" => RoundResult::Win,
                _ => unimplemented!(),
            };
            let p2_shape = select_round_result(&p1_shape, p2_round_res);
            let score = play_round((&p1_shape, &p2_shape));
            score.1
        })
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
