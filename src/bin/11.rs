use std::cell::RefCell;

use itertools::Itertools;

#[derive(Debug)]
enum Op {
    Add(u64),
    AddSelf,
    Multiply(u64),
    MultiplySelf,
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<Vec<u64>>,
    operation: Op,
    test_division: u64,
    next_monkeys: (usize, usize),
    inspected_count: RefCell<u64>,
}

impl Monkey {
    fn clear_items(&self) {
        *self.inspected_count.borrow_mut() += self.items.borrow().len() as u64;
        self.items.borrow_mut().clear();
    }
    fn add_items(&self, items: Vec<u64>) {
        self.items.borrow_mut().extend(items);
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let blocks = input.split("\n\n");
    let monkeys = blocks
        .map(|block| {
            let mut bl_lines = block.lines();
            let (_, items) = bl_lines.nth(1).unwrap().split_once(": ").unwrap();
            let items = items.split(", ").map(|item| item.parse::<u64>().unwrap());
            let op_line = bl_lines
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .collect_vec();
            let op = match (op_line[4], op_line[5]) {
                ("*", num) if num.parse::<u32>().is_ok() => Op::Multiply(num.parse().unwrap()),
                ("+", num) if num.parse::<u32>().is_ok() => Op::Add(num.parse().unwrap()),
                ("*", "old") => Op::MultiplySelf,
                ("+", "old") => Op::AddSelf,
                _ => unimplemented!(),
            };
            let test_division = bl_lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let next_monkeys: (usize, usize) = bl_lines
                .map(|line| {
                    line.split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            Monkey {
                items: RefCell::new(items.collect()),
                operation: op,
                test_division,
                next_monkeys,
                inspected_count: RefCell::new(0),
            }
        })
        .collect_vec();
    monkeys
}

fn monkeys_play<F>(monkeys: Vec<Monkey>, rounds: u64, worry_modifier: F) -> u64
where
    F: Fn(u64) -> u64,
{
    for _ in 0..rounds {
        for monkey in &monkeys {
            let mut throw_to_first = vec![];
            let mut throw_to_second = vec![];
            for item in monkey.items.borrow().iter() {
                let mut item = match monkey.operation {
                    Op::Add(num) => item + num,
                    Op::Multiply(num) => item * num,
                    Op::AddSelf => item + item,
                    Op::MultiplySelf => item * item,
                };
                item = worry_modifier(item);
                if item % monkey.test_division == 0 {
                    throw_to_first.push(item);
                } else {
                    throw_to_second.push(item);
                }
            }
            monkey.clear_items();
            monkeys[monkey.next_monkeys.0].add_items(throw_to_first);
            monkeys[monkey.next_monkeys.1].add_items(throw_to_second);
        }
    }
    let (a, b) = monkeys
        .iter()
        .map(|monkey| *monkey.inspected_count.borrow())
        .sorted()
        .rev()
        .next_tuple()
        .unwrap();
    a * b
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = parse_monkeys(input);
    let monkey_business_level = monkeys_play(monkeys, 20, |x| x / 3);
    Some(monkey_business_level)
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = parse_monkeys(input);
    let product = monkeys
        .iter()
        .map(|m| m.test_division as u64)
        .product::<u64>();
    let monkey_business_level = monkeys_play(monkeys, 10000, |x| x % product);
    Some(monkey_business_level)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
