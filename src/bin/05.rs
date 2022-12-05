#![feature(map_many_mut)]
#![feature(is_some_and)]
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

fn parse_input(input: &str, multiple: bool) -> HashMap<char, VecDeque<char>> {
    let mut stacks: HashMap<char, VecDeque<char>> = HashMap::new();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let (initial_stacks, ops) = input.split_once("\n\n").unwrap();

    let mut indexes = vec![];

    for (i, ch) in initial_stacks
        .lines()
        .rev()
        .next()
        .unwrap()
        .chars()
        .enumerate()
    {
        if ch.is_digit(10) {
            stacks.insert(ch, VecDeque::new());
            indexes.push(i);
        }
    }

    for line in initial_stacks.lines().rev().skip(1) {
        for i in &indexes {
            let ch = line.chars().nth(*i);
            if ch.is_some_and(|ch| ch != ' ') {
                let key = indexes.iter().position(|x| x == i).unwrap() + 1;
                let key = char::from_digit(key as u32, 10).unwrap();
                stacks
                    .entry(key)
                    .and_modify(|val| val.push_back(ch.unwrap()));
            }
        }
    }

    for op in ops.lines() {
        let captures = re.captures(op).unwrap();
        let cnt = captures.get(1).unwrap();
        let from = captures.get(2).unwrap();
        let to = captures.get(3).unwrap();

        let columns_to_mut = stacks
            .get_many_mut([
                &from.as_str().chars().next().unwrap(),
                &to.as_str().chars().next().unwrap(),
            ])
            .unwrap();

        let mut buffer = VecDeque::new();

        for _ in 0..cnt.as_str().parse::<usize>().unwrap() {
            let v = columns_to_mut[0].pop_back().unwrap();
            buffer.push_back(v);
        }

        for _ in 0..buffer.len() {
            if multiple {
                columns_to_mut[1].push_back(buffer.pop_back().unwrap());
            } else {
                columns_to_mut[1].push_back(buffer.pop_front().unwrap());
            }
        }
    }
    stacks
}

pub fn part_one(input: &str) -> Option<String> {
    let stacks = parse_input(input, false);
    let a = stacks
        .iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, vec_dec)| vec_dec.iter().last().unwrap())
        .collect::<String>();
    Some(a)
}

pub fn part_two(input: &str) -> Option<String> {
    let stacks = parse_input(input, true);
    let a = stacks
        .iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, vec_dec)| vec_dec.iter().last().unwrap())
        .collect::<String>();
    Some(a)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
